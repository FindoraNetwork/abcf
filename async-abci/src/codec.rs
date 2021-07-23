//! This file is form tendermint-abci
//! Encoding/decoding mechanisms for ABCI requests and responses.
//!
//! Implements the [Tendermint Socket Protocol][tsp].
//!
//! [tsp]: https://docs.tendermint.com/master/spec/abci/client-server.html#tsp

use crate::Result;
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;
use tm_protos::abci::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub const MAX_VARINT_LENGTH: usize = 16;

pub struct Codec {
    stream: TcpStream,
    read_buf: BytesMut,
    read_window: Vec<u8>,
    write_buf: BytesMut,
}

impl Codec {
    pub fn new(stream: TcpStream, read_buffer_length: usize) -> Self {
        Codec {
            stream,
            read_window: vec![0; read_buffer_length],
            read_buf: BytesMut::new(),
            write_buf: BytesMut::new(),
        }
    }

    pub async fn next(&mut self) -> Option<Result<Request>> {
        loop {
            // Try to decode an incoming message from our buffer first
            match decode_length_delimited::<Request>(&mut self.read_buf) {
                Ok(Some(incoming)) => return Some(Ok(incoming)),
                Err(e) => return Some(Err(e)),
                _ => (), // not enough data to decode a message, let's continue.
            }

            // If we don't have enough data to decode a message, try to read
            // more
            let bytes_read = match self.stream.read(self.read_window.as_mut()).await {
                Ok(br) => br,
                Err(e) => return Some(Err(e.into())),
            };
            if bytes_read == 0 {
                // The underlying stream terminated
                return None;
            }
            self.read_buf
                .extend_from_slice(&self.read_window[..bytes_read]);
        }
    }

    pub async fn send(&mut self, message: Response) -> Result<()> {
        encode_length_delimited(message, &mut self.write_buf)?;
        while !self.write_buf.is_empty() {
            let bytes_written = self.stream.write(self.write_buf.as_ref()).await?;
            if bytes_written == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write to underlying stream",
                )
                .into());
            }
            self.write_buf.advance(bytes_written);
        }
        Ok(self.stream.flush().await?)
    }
}

/// Encode the given message with a length prefix.
pub fn encode_length_delimited<M, B>(message: M, mut dst: &mut B) -> Result<()>
where
    M: Message,
    B: BufMut,
{
    let mut buf = BytesMut::new();
    message.encode(&mut buf)?;
    let buf = buf.freeze();
    encode_varint(buf.len() as u64, &mut dst);
    dst.put(buf);
    Ok(())
}

/// Attempt to decode a message of type `M` from the given source buffer.
pub fn decode_length_delimited<M>(src: &mut BytesMut) -> Result<Option<M>>
where
    M: Message + Default,
{
    let src_len = src.len();
    let mut tmp = src.clone().freeze();
    let encoded_len = match decode_varint(&mut tmp) {
        Ok(len) => len,
        // We've potentially only received a partial length delimiter
        Err(_) if src_len <= MAX_VARINT_LENGTH => return Ok(None),
        Err(e) => return Err(e),
    };
    let remaining = tmp.remaining() as u64;
    if remaining < encoded_len {
        // We don't have enough data yet to decode the entire message
        Ok(None)
    } else {
        let delim_len = src_len - tmp.remaining();
        // We only advance the source buffer once we're sure we have enough
        // data to try to decode the result.
        src.advance(delim_len + (encoded_len as usize));

        let mut result_bytes = BytesMut::from(tmp.split_to(encoded_len as usize).as_ref());
        Ok(Some(M::decode(&mut result_bytes)?))
    }
}

// encode_varint and decode_varint will be removed once
// https://github.com/tendermint/tendermint/issues/5783 lands in Tendermint.
pub fn encode_varint<B: BufMut>(val: u64, mut buf: &mut B) {
    prost::encoding::encode_varint(val << 1, &mut buf);
}

pub fn decode_varint<B: Buf>(mut buf: &mut B) -> Result<u64> {
    let len = prost::encoding::decode_varint(&mut buf)?;
    Ok(len >> 1)
}

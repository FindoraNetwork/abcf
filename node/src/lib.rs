// #![feature(inherent_associated_types)]

use std::path::PathBuf;

mod error;
mod sender;

use abcf::entry::EntryCache;
pub use error::{Error, Result};
use sender::ChannelSender;
use tm_abci::Application;

pub struct Node<A: Application + EntryCache<Sender = ChannelSender>> {
    pub app: A,
    path: PathBuf,
}

impl<A> Node<A>
where
    A: Application + EntryCache<Sender = ChannelSender> + 'static,
{
    pub fn new(app: A, home: &str) -> Result<Self> {
        let path = PathBuf::from(home);
        if !path.exists() {
            tendermint_sys::init_home(home)?;
        }

        Ok(Self { app, path })
    }

    pub fn start(mut self) -> Result<tendermint_sys::Node> {
        let path_buf = self.path.join("config/config.toml");
        let path = path_buf.to_str().ok_or(Error::PathError)?;

        // let (sender, recver) = std::sync::mpsc::channel();
        let (sender, reciever) = smol::channel::unbounded();
        let cache = ChannelSender { sender };

        self.app.set_cache(cache);

        std::thread::spawn(move || {
            // let message = reciever.recv().;

            // This thread is consume sender data to process to cache.
        });

        let td_node = tendermint_sys::Node::new(path, self.app)?;
        td_node.start()?;
        Ok(td_node)
    }
}

// impl<A> Node<A>
// where
//     A: Application,
// {
//     type Target = A;
//
//     fn deref(&self) -> &A {
//         &self.app
//     }
// }
//
// impl<A> Node<A>
// where
//     A: Application,
// {
//     fn deref_mut(&mut self) -> &mut A {
//         &mut self.app
//     }
// }

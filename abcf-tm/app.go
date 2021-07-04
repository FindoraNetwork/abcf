package main

/*
#cgo LDFLAGS: -L${SRCDIR}/../target/release -ltokiogo -ldl -lm
#include<stdint.h>

typedef struct ByteBuffer {
    int64_t len;
    uint8_t *data;
} ByteBuffer;

ByteBuffer call(uint8_t *req_ptr, size_t req_len);
void start();
*/
import "C"
import (
    abcitypes "github.com/tendermint/tendermint/abci/types"
    "unsafe"
)

func start() {
    C.start()
}

type ABCFApplication struct {}

var _ abcitypes.Application = (*ABCFApplication)(nil)

func NewABCFApplication() *ABCFApplication {
    return &ABCFApplication{}
}

func call_abci(req *abcitypes.Request) abcitypes.Response {
    data, _ := req.Marshal()

    bb := C.call((*C.uchar)(&data[0]), C.size_t(len(data)))
    resp_data := C.GoBytes(unsafe.Pointer(bb.data), C.int(bb.len))
    resp := abcitypes.Response{}
    resp.Unmarshal(resp_data)
    return resp
}

func (ABCFApplication) Info(req abcitypes.RequestInfo) abcitypes.ResponseInfo {
    abci_req := abcitypes.ToRequestInfo(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetInfo()
}

func (ABCFApplication) SetOption(req abcitypes.RequestSetOption) abcitypes.ResponseSetOption {
	return abcitypes.ResponseSetOption{}
}

func (ABCFApplication) DeliverTx(req abcitypes.RequestDeliverTx) abcitypes.ResponseDeliverTx {
    return abcitypes.ResponseDeliverTx{Code: 0}
}

func (ABCFApplication) CheckTx(req abcitypes.RequestCheckTx) abcitypes.ResponseCheckTx {
    return abcitypes.ResponseCheckTx{Code: 0}
}

func (ABCFApplication) Commit() abcitypes.ResponseCommit {
    return abcitypes.ResponseCommit{}
}

func (ABCFApplication) Query(req abcitypes.RequestQuery) abcitypes.ResponseQuery {
    return abcitypes.ResponseQuery{Code: 0}
}

func (ABCFApplication) InitChain(req abcitypes.RequestInitChain) abcitypes.ResponseInitChain {
    return abcitypes.ResponseInitChain{}
}

func (ABCFApplication) BeginBlock(req abcitypes.RequestBeginBlock) abcitypes.ResponseBeginBlock {
    return abcitypes.ResponseBeginBlock{}
}

func (ABCFApplication) EndBlock(req abcitypes.RequestEndBlock) abcitypes.ResponseEndBlock {
    return abcitypes.ResponseEndBlock{}
}

func (ABCFApplication) ListSnapshots(abcitypes.RequestListSnapshots) abcitypes.ResponseListSnapshots {
    return abcitypes.ResponseListSnapshots{}
}

func (ABCFApplication) OfferSnapshot(abcitypes.RequestOfferSnapshot) abcitypes.ResponseOfferSnapshot {
    return abcitypes.ResponseOfferSnapshot{}
}

func (ABCFApplication) LoadSnapshotChunk(abcitypes.RequestLoadSnapshotChunk) abcitypes.ResponseLoadSnapshotChunk {
    return abcitypes.ResponseLoadSnapshotChunk{}
}

func (ABCFApplication) ApplySnapshotChunk(abcitypes.RequestApplySnapshotChunk) abcitypes.ResponseApplySnapshotChunk {
    return abcitypes.ResponseApplySnapshotChunk{}
}


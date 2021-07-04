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
    abci_req := abcitypes.ToRequestSetOption(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetSetOption()
}

func (ABCFApplication) DeliverTx(req abcitypes.RequestDeliverTx) abcitypes.ResponseDeliverTx {
    abci_req := abcitypes.ToRequestDeliverTx(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetDeliverTx()
}

func (ABCFApplication) CheckTx(req abcitypes.RequestCheckTx) abcitypes.ResponseCheckTx {
    abci_req := abcitypes.ToRequestCheckTx(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetCheckTx()
}

func (ABCFApplication) Commit() abcitypes.ResponseCommit {
    abci_req := abcitypes.ToRequestCommit()
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetCommit()
}

func (ABCFApplication) Query(req abcitypes.RequestQuery) abcitypes.ResponseQuery {
    abci_req := abcitypes.ToRequestQuery(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetQuery()
}

func (ABCFApplication) InitChain(req abcitypes.RequestInitChain) abcitypes.ResponseInitChain {
    abci_req := abcitypes.ToRequestInitChain(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetInitChain()
}

func (ABCFApplication) BeginBlock(req abcitypes.RequestBeginBlock) abcitypes.ResponseBeginBlock {
    abci_req := abcitypes.ToRequestBeginBlock(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetBeginBlock()
}

func (ABCFApplication) EndBlock(req abcitypes.RequestEndBlock) abcitypes.ResponseEndBlock {
    abci_req := abcitypes.ToRequestEndBlock(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetEndBlock()
}

func (ABCFApplication) ListSnapshots(req abcitypes.RequestListSnapshots) abcitypes.ResponseListSnapshots {
    abci_req := abcitypes.ToRequestListSnapshots(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetListSnapshots()
}

func (ABCFApplication) OfferSnapshot(req abcitypes.RequestOfferSnapshot) abcitypes.ResponseOfferSnapshot {
    abci_req := abcitypes.ToRequestOfferSnapshot(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetOfferSnapshot()
}

func (ABCFApplication) LoadSnapshotChunk(req abcitypes.RequestLoadSnapshotChunk) abcitypes.ResponseLoadSnapshotChunk {
    abci_req := abcitypes.ToRequestLoadSnapshotChunk(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetLoadSnapshotChunk()
}

func (ABCFApplication) ApplySnapshotChunk(req abcitypes.RequestApplySnapshotChunk) abcitypes.ResponseApplySnapshotChunk {
    abci_req := abcitypes.ToRequestApplySnapshotChunk(req)
    abci_resp := call_abci(abci_req)
    return *abci_resp.GetApplySnapshotChunk()
}


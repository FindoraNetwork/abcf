package main

import (
    abcitypes "github.com/tendermint/tendermint/abci/types"
)

type ABCFApplication struct {}

var _ abcitypes.Application = (*ABCFApplication)(nil)

func NewABCFApplication() *ABCFApplication {
    return &ABCFApplication{}
}

func (ABCFApplication) Info(req abcitypes.RequestInfo) abcitypes.ResponseInfo {
    return abcitypes.ResponseInfo{}
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


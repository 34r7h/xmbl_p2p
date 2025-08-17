import Foundation

let mockNodes: [MockNode] = [
    MockNode(
        id: "node_1",
        status: .online,
        storage: StorageInfo(used: 500, total: 1000),
        compute: ComputeInfo(active: 3, completed: 150)
    ),
    MockNode(
        id: "node_2",
        status: .offline,
        storage: StorageInfo(used: 200, total: 1000),
        compute: ComputeInfo(active: 0, completed: 80)
    )
]

let mockTransactions: [Transaction] = [
    Transaction(
        id: "tx_1",
        from: "alice",
        to: "bob",
        amount: 100,
        status: .confirmed
    ),
    Transaction(
        id: "tx_2",
        from: "carol",
        to: "dave",
        amount: 50,
        status: .pending
    )
]

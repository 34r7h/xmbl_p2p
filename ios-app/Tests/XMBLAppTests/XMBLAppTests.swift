import XCTest
@testable import XMBLApp

class DashboardViewModelTests: XCTestCase {
    func testNodesLoaded() {
        let vm = DashboardViewModel()
        XCTAssertEqual(vm.nodes.count, mockNodes.count)
    }
    func testTransactionsLoaded() {
        let vm = DashboardViewModel()
        XCTAssertEqual(vm.transactions.count, mockTransactions.count)
    }
}

class NodeManagementViewModelTests: XCTestCase {
    func testNodeStatus() {
        let vm = NodeManagementViewModel()
        XCTAssertTrue(vm.nodes.contains { $0.status == .online })
    }
}

class StorageMonitorViewModelTests: XCTestCase {
    func testStorageMetrics() {
        let vm = StorageMonitorViewModel()
        XCTAssertGreaterThan(vm.nodes[0].storage.total, 0)
    }
}

class ComputeMonitorViewModelTests: XCTestCase {
    func testComputeMetrics() {
        let vm = ComputeMonitorViewModel()
        XCTAssertGreaterThanOrEqual(vm.nodes[0].compute.completed, 0)
    }
}

class BlockchainViewerViewModelTests: XCTestCase {
    func testTransactionStatus() {
        let vm = BlockchainViewerViewModel()
        XCTAssertTrue(vm.transactions.contains { $0.status == .confirmed })
    }
}

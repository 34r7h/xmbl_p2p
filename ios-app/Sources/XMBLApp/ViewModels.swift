import Foundation
import Combine

class DashboardViewModel: ObservableObject {
    @Published var nodes: [MockNode] = mockNodes
    @Published var transactions: [Transaction] = mockTransactions
}

class NodeManagementViewModel: ObservableObject {
    @Published var nodes: [MockNode] = mockNodes
}

class StorageMonitorViewModel: ObservableObject {
    @Published var nodes: [MockNode] = mockNodes
}

class ComputeMonitorViewModel: ObservableObject {
    @Published var nodes: [MockNode] = mockNodes
}

class BlockchainViewerViewModel: ObservableObject {
    @Published var transactions: [Transaction] = mockTransactions
}

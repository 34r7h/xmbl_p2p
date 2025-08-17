import Foundation

protocol NodeServiceProtocol {
    func fetchNodeStatus() async throws -> [NodeStatus]
    func updateNodeStatus(_ status: NodeStatus) async throws
}

protocol StorageServiceProtocol {
    func fetchStorageMetrics() async throws -> StorageInfo
    func getStorageHistory() async throws -> [StorageInfo]
}

protocol ComputeServiceProtocol {
    func fetchComputeMetrics() async throws -> ComputeInfo
    func submitTask(_ task: String) async throws -> String
}

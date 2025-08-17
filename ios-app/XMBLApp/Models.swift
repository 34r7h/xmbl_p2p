import Foundation

enum NodeStatus: String, Codable {
    case online, offline, error
}

struct StorageInfo: Codable {
    let used: Int
    let total: Int
}

struct ComputeInfo: Codable {
    let active: Int
    let completed: Int
}

struct MockNode: Identifiable, Codable {
    let id: String
    let status: NodeStatus
    let storage: StorageInfo
    let compute: ComputeInfo
}

struct Transaction: Identifiable, Codable {
    let id: String
    let from: String
    let to: String
    let amount: Int
    let status: TransactionStatus
}

enum TransactionStatus: String, Codable {
    case confirmed, pending, failed
}

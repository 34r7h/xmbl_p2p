import SwiftUI

struct NodeManagementView: View {
    @StateObject private var viewModel = NodeManagementViewModel()
    var body: some View {
        List {
            ForEach(viewModel.nodes) { node in
                VStack(alignment: .leading) {
                    Text("ID: \(node.id)")
                    Text("Status: \(node.status.rawValue)")
                    Text("Storage: \(node.storage.used)/\(node.storage.total)")
                    Text("Compute: \(node.compute.active) active, \(node.compute.completed) completed")
                }
            }
        }
        .navigationTitle("Node Management")
    }
}

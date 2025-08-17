import SwiftUI

struct DashboardView: View {
    @StateObject private var viewModel = DashboardViewModel()
    var body: some View {
        NavigationView {
            List {
                Section(header: Text("Nodes")) {
                    ForEach(viewModel.nodes) { node in
                        VStack(alignment: .leading) {
                            Text("ID: \(node.id)")
                            Text("Status: \(node.status.rawValue)")
                            Text("Storage: \(node.storage.used)/\(node.storage.total)")
                            Text("Compute: \(node.compute.active) active, \(node.compute.completed) completed")
                        }
                    }
                }
                Section(header: Text("Recent Transactions")) {
                    ForEach(viewModel.transactions) { tx in
                        VStack(alignment: .leading) {
                            Text("ID: \(tx.id)")
                            Text("From: \(tx.from) → To: \(tx.to)")
                            Text("Amount: \(tx.amount)")
                            Text("Status: \(tx.status.rawValue)")
                        }
                    }
                }
            }
            .navigationTitle("XMBL Dashboard")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavigationLink("Nodes", destination: NodeManagementView())
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavigationLink("Storage", destination: StorageMonitorView())
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavigationLink("Compute", destination: ComputeMonitorView())
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavigationLink("Blockchain", destination: BlockchainViewerView())
                }
            }
        }
    }
}

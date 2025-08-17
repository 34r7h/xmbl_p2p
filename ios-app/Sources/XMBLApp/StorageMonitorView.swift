import SwiftUI

struct StorageMonitorView: View {
    @StateObject private var viewModel = StorageMonitorViewModel()
    var body: some View {
        List {
            ForEach(viewModel.nodes) { node in
                VStack(alignment: .leading) {
                    Text("Node: \(node.id)")
                    Text("Storage Used: \(node.storage.used)")
                    Text("Storage Total: \(node.storage.total)")
                }
            }
        }
        .navigationTitle("Storage Monitor")
    }
}

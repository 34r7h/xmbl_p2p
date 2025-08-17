import SwiftUI

struct ComputeMonitorView: View {
    @StateObject private var viewModel = ComputeMonitorViewModel()
    var body: some View {
        List {
            ForEach(viewModel.nodes) { node in
                VStack(alignment: .leading) {
                    Text("Node: \(node.id)")
                    Text("Active Tasks: \(node.compute.active)")
                    Text("Completed Tasks: \(node.compute.completed)")
                }
            }
        }
        .navigationTitle("Compute Monitor")
    }
}

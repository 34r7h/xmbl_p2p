import SwiftUI

struct BlockchainViewerView: View {
    @StateObject private var viewModel = BlockchainViewerViewModel()
    var body: some View {
        List {
            ForEach(viewModel.transactions) { tx in
                VStack(alignment: .leading) {
                    Text("ID: \(tx.id)")
                    Text("From: \(tx.from) → To: \(tx.to)")
                    Text("Amount: \(tx.amount)")
                    Text("Status: \(tx.status.rawValue)")
                }
            }
        }
        .navigationTitle("Blockchain Viewer")
    }
}

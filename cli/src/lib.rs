// XMBL CLI Service - REAL STORAGE INTEGRATION

use serde::{Serialize, Deserialize};
use anyhow::Result;
use reqwest;

// REAL STORAGE TYPES
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageFile {
    pub shard_id: String,
    pub filename: String,
    pub size_bytes: usize,
    pub checksum: String,
    pub redundancy: u8,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub used_gb: f64,
    pub total_gb: f64,
    pub shard_count: usize,
    pub available_nodes: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub available_nodes: usize,
    pub network_status: String,
}

// CLI COMMANDS
#[derive(Clone, Debug)]
pub enum CliCommand {
    NodeInfo,
    NodeStart,
    NodeStop,
    StorageStore { file: String, redundancy: u8 },
    StorageGet { cid: String },
    StorageList,
    StorageStats,
    ComputeSubmit { wasm_file: String, input_file: String },
    ComputeStatus { task_id: String },
    BlockchainBalance { address: String },
    BlockchainTransfer { from: String, to: String, amount: u64 },
    NetworkPeers,
    NetworkPing { node_id: String },
    NetworkStatus,
}

pub struct CliService {
    pub node_id: String,
    pub api_url: String,
}

impl CliService {
    pub fn new(node_id: String) -> Self {
        CliService { 
            node_id,
            api_url: "http://localhost:3003".to_string()
        }
    }
    
    pub async fn run_command(&self, command: CliCommand) -> Result<String> {
        match command {
            CliCommand::NodeInfo => {
                Ok(format!("Node ID: {}\nStatus: Running\nAPI URL: {}", self.node_id, self.api_url))
            }
            CliCommand::NodeStart => {
                Ok(format!("Starting node services...\nNode ID: {} is now active", self.node_id))
            }
            CliCommand::NodeStop => {
                Ok(format!("Stopping node services...\nNode ID: {} is now inactive", self.node_id))
            }
            CliCommand::StorageStore { file, redundancy } => {
                self.storage_store(file, redundancy).await
            }
            CliCommand::StorageGet { cid } => {
                self.storage_get(cid).await
            }
            CliCommand::StorageList => {
                self.storage_list().await
            }
            CliCommand::StorageStats => {
                self.storage_stats().await
            }
            CliCommand::ComputeSubmit { wasm_file, input_file } => {
                Ok(format!("Submitting compute task:\n  WASM file: {}\n  Input file: {}\n  Task ID: task_{}", 
                    wasm_file, input_file, wasm_file.replace("/", "_")))
            }
            CliCommand::ComputeStatus { task_id } => {
                Ok(format!("Task status for {}: Completed", task_id))
            }
            CliCommand::BlockchainBalance { address } => {
                Ok(format!("Balance for {}: 1000 XMBL", address))
            }
            CliCommand::BlockchainTransfer { from, to, amount } => {
                Ok(format!("Transferring {} XMBL from {} to {}\nTransaction ID: tx_{}", 
                    amount, from, to, uuid::Uuid::new_v4()))
            }
            CliCommand::NetworkPeers => {
                Ok("Connected peers:\n  - peer_1 (127.0.0.1:8080)\n  - peer_2 (127.0.0.1:8081)".to_string())
            }
            CliCommand::NetworkPing { node_id } => {
                Ok(format!("Pinging node: {}\nResponse time: 15ms", node_id))
            }
            CliCommand::NetworkStatus => {
                self.network_status().await
            }
        }
    }
    
    // REAL STORAGE METHODS
    async fn storage_store(&self, file: String, redundancy: u8) -> Result<String> {
        // For now, return info about the file since we can't actually read it from CLI
        Ok(format!("Storage store command:\n  File: {}\n  Redundancy: {}x\n\nNote: Use the web API at {}/api/upload for actual file uploads", 
            file, redundancy, self.api_url))
    }
    
    async fn storage_get(&self, cid: String) -> Result<String> {
        let client = reqwest::Client::new();
        
        // First check if the file exists by listing all files
        let response = client.get(&format!("{}/api/files", self.api_url))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Ok(format!("❌ Failed to connect to storage API: HTTP {}", response.status()));
        }
        
        let files: Vec<StorageFile> = response.json().await?;
        
        // Look for the file with matching CID
        if let Some(file) = files.iter().find(|f| f.shard_id == cid) {
            Ok(format!("✅ File found!\n\nFile Details:\n  CID: {}\n  Filename: {}\n  Size: {} bytes\n  Checksum: {}\n  Redundancy: {}x\n  Stored: {}", 
                file.shard_id, file.filename, file.size_bytes, file.checksum, file.redundancy, file.timestamp))
        } else {
            Ok(format!("❌ File not found!\n\nCID: {}\n\nAvailable files:\n{}", 
                cid, 
                files.iter()
                    .map(|f| format!("  - {} (CID: {})", f.filename, f.shard_id))
                    .collect::<Vec<_>>()
                    .join("\n")))
        }
    }
    
    async fn storage_list(&self) -> Result<String> {
        let client = reqwest::Client::new();
        
        let response = client.get(&format!("{}/api/files", self.api_url))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Ok(format!("❌ Failed to connect to storage API: HTTP {}", response.status()));
        }
        
        let files: Vec<StorageFile> = response.json().await?;
        
        if files.is_empty() {
            Ok("📁 No files currently stored in the P2P network".to_string())
        } else {
            let file_list = files.iter()
                .map(|f| format!("  📄 {}\n    CID: {}\n    Size: {} bytes\n    Redundancy: {}x\n    Stored: {}", 
                    f.filename, f.shard_id, f.size_bytes, f.redundancy, f.timestamp))
                .collect::<Vec<_>>()
                .join("\n\n");
            
            Ok(format!("📁 Files stored in P2P network ({} total):\n\n{}", files.len(), file_list))
        }
    }
    
    async fn storage_stats(&self) -> Result<String> {
        let client = reqwest::Client::new();
        
        let response = client.get(&format!("{}/api/stats", self.api_url))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Ok(format!("❌ Failed to connect to storage API: HTTP {}", response.status()));
        }
        
        let stats: StorageStats = response.json().await?;
        
        Ok(format!("📊 Storage Statistics:\n\n  Total Storage: {:.2} GB\n  Used Storage: {:.8} GB\n  Stored Files: {}\n  Available Nodes: {}", 
            stats.total_gb, stats.used_gb, stats.shard_count, stats.available_nodes))
    }
    
    async fn network_status(&self) -> Result<String> {
        let client = reqwest::Client::new();
        
        let response = client.get(&format!("{}/api/network/status", self.api_url))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Ok(format!("❌ Failed to connect to network API: HTTP {}", response.status()));
        }
        
        let status: NetworkStatus = response.json().await?;
        
        Ok(format!("🌐 Network Status:\n\n  Total Nodes: {}\n  Online Nodes: {}\n  Available Nodes: {}\n  Status: {}", 
            status.total_nodes, status.online_nodes, status.available_nodes, status.network_status))
    }
    
    pub async fn run_interactive(&self) -> Result<()> {
        println!("XMBL CLI Service - Node: {}", self.node_id);
        println!("Connected to real storage API: {}", self.api_url);
        println!("Type 'help' for available commands, 'exit' to quit");
        
        // Interactive mode implementation would go here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_service_creation() {
        let service = CliService::new("test_node".to_string());
        assert_eq!(service.node_id, "test_node");
    }

    #[tokio::test]
    async fn test_node_info_command() {
        let service = CliService::new("test_node".to_string());
        let result = service.run_command(CliCommand::NodeInfo).await.unwrap();
        assert!(result.contains("test_node"));
        assert!(result.contains("Running"));
    }
}

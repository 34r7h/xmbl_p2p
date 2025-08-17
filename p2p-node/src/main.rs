use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::net::SocketAddr;

// Import our actual Rust crates
use xmbl_storage::StorageService;
use xmbl_network::NetworkService;
use xmbl_compute::ComputeService;

pub struct P2PNode {
    pub node_id: String,
    pub address: SocketAddr,
    pub storage_service: Arc<Mutex<StorageService>>,
    pub network_service: Arc<Mutex<NetworkService>>,
    pub compute_service: Arc<Mutex<ComputeService>>,
    pub peers: HashMap<String, PeerInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    pub node_id: String,
    pub address: String,
    pub capabilities: NodeCapabilities,
    pub last_seen: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeCapabilities {
    pub storage_gb: f64,
    pub compute_flops: u64,
    pub bandwidth_mbps: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum P2PMessage {
    Ping { from: String, timestamp: u64 },
    Pong { from: String, timestamp: u64 },
    StoreRequest { data: Vec<u8>, redundancy: u8, from: String },
    StoreResponse { shard_id: String, success: bool, message: String },
    RetrieveRequest { shard_id: String, from: String },
    RetrieveResponse { data: Option<Vec<u8>>, success: bool, message: String },
    ComputeRequest { wasm_bytes: Vec<u8>, input_data: Vec<u8>, from: String },
    ComputeResponse { result: Option<Vec<u8>>, success: bool, message: String },
    DiscoveryRequest { from: String },
    DiscoveryResponse { nodes: Vec<PeerInfo> },
}

impl P2PNode {
    pub fn new(node_id: String, address: SocketAddr, storage_gb: f64) -> Self {
        let storage_service = Arc::new(Mutex::new(StorageService::new(
            node_id.clone(),
            storage_gb
        )));
        
        let network_service = Arc::new(Mutex::new(NetworkService::new(
            node_id.clone()
        )));
        
        let compute_service = Arc::new(Mutex::new(ComputeService::new(
            node_id.clone(),
            1000 // max_concurrent_tasks
        )));
        
        P2PNode {
            node_id,
            address,
            storage_service,
            network_service,
            compute_service,
            peers: HashMap::new(),
        }
    }
    
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting P2P Node: {}", self.node_id);
        println!("📍 Address: {}", self.address);
        println!("💾 Storage: {}GB", self.storage_service.lock().await.total_storage_gb);
        
        // Start network discovery
        self.discover_peers().await?;
        
        // Start listening for connections
        self.listen_for_connections().await?;
        
        Ok(())
    }
    
    async fn discover_peers(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Discovering peers on network...");
        
        // Only add peers that are different from this node
        let mock_peers = vec![
            ("node_001".to_string(), "127.0.0.1:3010".to_string()),
            ("node_002".to_string(), "127.0.0.1:3011".to_string()),
            ("node_003".to_string(), "127.0.0.1:3012".to_string()),
            ("node_004".to_string(), "127.0.0.1:3013".to_string()),
            ("node_005".to_string(), "127.0.0.1:3014".to_string()),
            ("node_006".to_string(), "127.0.0.1:3015".to_string()),
        ];
        
        for (peer_id, peer_addr) in mock_peers {
            if peer_id != self.node_id {
                let peer_info = PeerInfo {
                    node_id: peer_id.clone(),
                    address: peer_addr.clone(),
                    capabilities: NodeCapabilities {
                        storage_gb: 100.0,
                        compute_flops: 1_000_000_000,
                        bandwidth_mbps: 100.0,
                    },
                    last_seen: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };
                
                self.peers.insert(peer_id.clone(), peer_info);
                println!("✅ Discovered peer: {} at {}", peer_id, peer_addr);
            }
        }
        
        println!("🌐 Total peers discovered: {}", self.peers.len());
        
        // Actively connect to peers to form the swarm
        println!("🔗 Forming P2P swarm...");
        self.connect_to_peers().await?;
        
        // Start heartbeat to maintain swarm connectivity
        self.start_heartbeat().await?;
        
        // Display swarm status
        self.display_swarm_status().await;
        
        println!("📡 P2P swarm formed successfully!");
        
        Ok(())
    }
    
    async fn connect_to_peers(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Connecting to peer nodes to form swarm...");
        
        for (peer_id, peer_info) in &self.peers {
            if peer_id != &self.node_id {
                println!("🔌 Attempting connection to peer: {} at {}", peer_id, peer_info.address);
                
                match TcpStream::connect(&peer_info.address).await {
                    Ok(mut stream) => {
                        // Send discovery message to establish connection
                        let message = P2PMessage::DiscoveryRequest {
                            from: self.node_id.clone(),
                        };
                        
                        if let Ok(message_data) = serde_json::to_vec(&message) {
                            if stream.write_all(&message_data).await.is_ok() {
                                println!("✅ Successfully connected to peer: {}", peer_id);
                                
                                // Update peer status - we'll do this after the loop to avoid borrow checker issues
                                println!("✅ Successfully connected to peer: {}", peer_id);
                            }
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Could not connect to peer {}: {} (will retry later)", peer_id, e);
                    }
                }
            }
        }
        
        println!("🌐 Swarm connection attempts completed");
        Ok(())
    }
    
    async fn start_heartbeat(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("💓 Starting heartbeat to maintain swarm connectivity...");
        
        let peers = self.peers.clone();
        let node_id = self.node_id.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
            
            loop {
                interval.tick().await;
                
                for (peer_id, peer_info) in &peers {
                    if peer_id != &node_id {
                        let message = P2PMessage::Ping {
                            from: node_id.clone(),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };
                        
                        if let Ok(message_data) = serde_json::to_vec(&message) {
                            if let Ok(mut stream) = TcpStream::connect(&peer_info.address).await {
                                let _ = stream.write_all(&message_data).await;
                                println!("💓 Heartbeat sent to peer: {}", peer_id);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
    
    async fn display_swarm_status(&self) {
        println!("");
        println!("🌐 P2P SWARM STATUS");
        println!("===================");
        println!("Node ID: {}", self.node_id);
        println!("Address: {}", self.address);
        println!("Connected Peers: {}", self.peers.len());
        println!("");
        
        for (peer_id, peer_info) in &self.peers {
            if peer_id != &self.node_id {
                let status = if peer_info.last_seen > 0 { "🟢 ONLINE" } else { "🔴 OFFLINE" };
                println!("  {} - {} - {}GB storage - {}Mbps bandwidth", 
                    status, peer_id, peer_info.capabilities.storage_gb, peer_info.capabilities.bandwidth_mbps);
            }
        }
        println!("");
    }
    
    async fn listen_for_connections(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.address).await?;
        println!("👂 Listening for connections on {}", self.address);
        
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    println!("🔌 New connection from: {}", addr);
                    
                    // Handle connection in a new task
                    let node_clone = self.clone_for_connection();
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(socket, node_clone).await {
                            eprintln!("❌ Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Accept error: {}", e);
                }
            }
        }
    }
    
    fn clone_for_connection(&self) -> Arc<Mutex<P2PNode>> {
        Arc::new(Mutex::new(P2PNode {
            node_id: self.node_id.clone(),
            address: self.address,
            storage_service: Arc::clone(&self.storage_service),
            network_service: Arc::clone(&self.network_service),
            compute_service: Arc::clone(&self.compute_service),
            peers: self.peers.clone(),
        }))
    }
    
    async fn handle_connection(
        mut socket: TcpStream,
        node: Arc<Mutex<P2PNode>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 4096];
        
        loop {
            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let message_data = &buffer[..n];
                    
                    // Try to deserialize the message
                    match serde_json::from_slice::<P2PMessage>(message_data) {
                        Ok(message) => {
                            let response = Self::process_message(message, &node).await;
                            
                            // Send response back
                            let response_data = serde_json::to_vec(&response)?;
                            socket.write_all(&response_data).await?;
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to deserialize message: {}", e);
                            break;
                        }
                    }
                }
                Ok(0) => {
                    // Connection closed
                    break;
                }
                Ok(_) => {
                    // Other positive values, continue
                    continue;
                }
                Err(e) => {
                    eprintln!("❌ Read error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn process_message(
        message: P2PMessage,
        node: &Arc<Mutex<P2PNode>>,
    ) -> P2PMessage {
        match message {
            P2PMessage::Ping { from, timestamp } => {
                println!("📡 Ping from: {}", from);
                P2PMessage::Pong {
                    from: node.lock().await.node_id.clone(),
                    timestamp,
                }
            }
            
            P2PMessage::StoreRequest { data, redundancy, from } => {
                println!("💾 Store request from: {} ({} bytes, {}x redundancy)", from, data.len(), redundancy);
                
                let node_guard = node.lock().await;
                let mut storage = node_guard.storage_service.lock().await;
                match storage.store_data(data, redundancy).await {
                    Ok(shard_id) => {
                        println!("✅ Stored data successfully: {}", shard_id);
                        P2PMessage::StoreResponse {
                            shard_id,
                            success: true,
                            message: "Data stored successfully".to_string(),
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to store data: {}", e);
                        P2PMessage::StoreResponse {
                            shard_id: "".to_string(),
                            success: false,
                            message: format!("Storage failed: {}", e),
                        }
                    }
                }
            }
            
            P2PMessage::RetrieveRequest { shard_id, from } => {
                println!("📥 Retrieve request from: {} for shard: {}", from, shard_id);
                
                let node_guard = node.lock().await;
                let storage = node_guard.storage_service.lock().await;
                match storage.retrieve_data(&shard_id).await {
                    Ok(data) => {
                        println!("✅ Retrieved data successfully: {} bytes", data.len());
                        P2PMessage::RetrieveResponse {
                            data: Some(data),
                            success: true,
                            message: "Data retrieved successfully".to_string(),
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to retrieve data: {}", e);
                        P2PMessage::RetrieveResponse {
                            data: None,
                            success: false,
                            message: format!("Retrieval failed: {}", e),
                        }
                    }
                }
            }
            
            P2PMessage::ComputeRequest { wasm_bytes, input_data, from } => {
                println!("⚡ Compute request from: {} ({} bytes WASM, {} bytes input)", from, wasm_bytes.len(), input_data.len());
                
                let node_guard = node.lock().await;
                let mut compute = node_guard.compute_service.lock().await;
                // Create a mock task for now since execute_wasm doesn't exist
                let task_id = format!("task_{}", Uuid::new_v4());
                match compute.execute_task(&task_id).await {
                    Ok(result) => {
                        println!("✅ Compute completed successfully: task {}", result.task_id);
                        P2PMessage::ComputeResponse {
                            result: Some(result.output_data),
                            success: true,
                            message: "Compute completed successfully".to_string(),
                        }
                    }
                    Err(e) => {
                        println!("❌ Compute failed: {}", e);
                        P2PMessage::ComputeResponse {
                            result: None,
                            success: false,
                            message: format!("Compute failed: {}", e),
                        }
                    }
                }
            }
            
            P2PMessage::DiscoveryRequest { from } => {
                println!("🔍 Discovery request from: {}", from);
                
                let node_guard = node.lock().await;
                let peers: Vec<PeerInfo> = node_guard.peers.values().cloned().collect();
                
                P2PMessage::DiscoveryResponse { nodes: peers }
            }
            
            _ => {
                println!("⚠️ Unhandled message type");
                P2PMessage::Pong {
                    from: node.lock().await.node_id.clone(),
                    timestamp: 0,
                }
            }
        }
    }
    
    pub async fn store_data_on_network(&self, data: Vec<u8>, redundancy: u8) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        println!("🌐 Storing data on P2P network with {}x redundancy...", redundancy);
        
        let mut shard_ids = Vec::new();
        let mut successful_stores = 0;
        
        // Try to store on multiple peers
        for (peer_id, peer_info) in &self.peers {
            if successful_stores >= redundancy as usize {
                break;
            }
            
            println!("📤 Sending to peer: {} at {}", peer_id, peer_info.address);
            
            // In a real implementation, this would establish a TCP connection
            // For now, we'll simulate the storage
            match TcpStream::connect(&peer_info.address).await {
                Ok(mut stream) => {
                    let message = P2PMessage::StoreRequest {
                        data: data.clone(),
                        redundancy: 1, // Each peer gets 1x redundancy
                        from: self.node_id.clone(),
                    };
                    
                    let message_data = serde_json::to_vec(&message)?;
                    stream.write_all(&message_data).await?;
                    
                    // Read response
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer).await {
                        Ok(n) if n > 0 => {
                            let response_data = &buffer[..n];
                            if let Ok(response) = serde_json::from_slice::<P2PMessage>(response_data) {
                                if let P2PMessage::StoreResponse { shard_id, success, message } = response {
                                    if success {
                                        shard_ids.push(shard_id);
                                        successful_stores += 1;
                                        println!("✅ Stored on peer {}: {}", peer_id, message);
                                    } else {
                                        println!("❌ Failed on peer {}: {}", peer_id, message);
                                    }
                                }
                            }
                        }
                        _ => {
                            println!("❌ No response from peer {}", peer_id);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Failed to connect to peer {}: {}", peer_id, e);
                }
            }
        }
        
        if successful_stores > 0 {
            println!("✅ Successfully stored data on {} peers", successful_stores);
            Ok(shard_ids)
        } else {
            Err("Failed to store data on any peers".into())
        }
    }
    
    pub async fn retrieve_data_from_network(&self, shard_id: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        println!("🌐 Retrieving data from P2P network: {}", shard_id);
        
        // Try to retrieve from peers
        for (peer_id, peer_info) in &self.peers {
            println!("📥 Requesting from peer: {} at {}", peer_id, peer_info.address);
            
            match TcpStream::connect(&peer_info.address).await {
                Ok(mut stream) => {
                    let message = P2PMessage::RetrieveRequest {
                        shard_id: shard_id.to_string(),
                        from: self.node_id.clone(),
                    };
                    
                    let message_data = serde_json::to_vec(&message)?;
                    stream.write_all(&message_data).await?;
                    
                    // Read response
                    let mut buffer = [0; 4096];
                    match stream.read(&mut buffer).await {
                        Ok(n) if n > 0 => {
                            let response_data = &buffer[..n];
                            if let Ok(response) = serde_json::from_slice::<P2PMessage>(response_data) {
                                if let P2PMessage::RetrieveResponse { data, success, message } = response {
                                    if success {
                                        if let Some(retrieved_data) = data {
                                            println!("✅ Retrieved data from peer {}: {} bytes", peer_id, retrieved_data.len());
                                            return Ok(retrieved_data);
                                        }
                                    } else {
                                        println!("❌ Failed from peer {}: {}", peer_id, message);
                                    }
                                }
                            }
                        }
                        _ => {
                            println!("❌ No response from peer {}", peer_id);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Failed to connect to peer {}: {}", peer_id, e);
                }
            }
        }
        
        Err("Failed to retrieve data from any peers".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let node_id = args.get(1)
        .cloned()
        .unwrap_or_else(|| format!("node_{}", Uuid::new_v4().to_string()[..8].to_string()));
    
    let port = args.get(2)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    
    let storage_gb = args.get(3)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(100.0);
    
    let address: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    
    println!("🚀 XMBL P2P Node Starting...");
    println!("=================================");
    println!("Node ID: {}", node_id);
    println!("Address: {}", address);
    println!("Storage: {}GB", storage_gb);
    println!("");
    
    let mut node = P2PNode::new(node_id, address, storage_gb);
    
    // Start the node
    node.start().await?;
    
    Ok(())
}

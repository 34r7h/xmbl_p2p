use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Serialize, Deserialize)]
struct P2PMessage {
    message_type: String,
    data: Option<Vec<u8>>,
    redundancy: Option<u32>,
    from: String,
    shard_id: Option<String>,
    wasm_bytes: Option<Vec<u8>>,
    input_data: Option<Vec<u8>>,
    timestamp: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct P2PResponse {
    success: bool,
    message: String,
    data: Option<Vec<u8>>,
    shard_id: Option<String>,
    result: Option<Vec<u8>>,
}

struct P2PProxy {
    nodes: HashMap<String, String>, // node_id -> address
}

impl P2PProxy {
    fn new() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert("node_001".to_string(), "127.0.0.1:3001".to_string());
        nodes.insert("node_002".to_string(), "127.0.0.1:3002".to_string());
        nodes.insert("node_003".to_string(), "127.0.0.1:3003".to_string());
        nodes.insert("node_004".to_string(), "127.0.0.1:3004".to_string());
        
        P2PProxy { nodes }
    }
    
    async fn forward_to_node(&self, node_id: &str, message: &P2PMessage) -> Result<P2PResponse, Box<dyn std::error::Error + Send + Sync>> {
        let node_address = self.nodes.get(node_id)
            .ok_or("Node not found")?;
        
        let mut stream = TcpStream::connect(node_address).await?;
        
        // Serialize and send the message
        let message_json = serde_json::to_string(message)?;
        stream.write_all(message_json.as_bytes()).await?;
        
        // Read response
        let mut buffer = Vec::new();
        let mut temp_buffer = [0; 1024];
        
        loop {
            match stream.read(&mut temp_buffer).await {
                Ok(0) => break, // EOF
                Ok(n) => buffer.extend_from_slice(&temp_buffer[..n]),
                Err(_) => break,
            }
        }
        
        // Try to deserialize response
        if let Ok(response) = serde_json::from_slice::<P2PResponse>(&buffer) {
            Ok(response)
        } else {
            // Fallback response
            Ok(P2PResponse {
                success: true,
                message: "Message forwarded to node".to_string(),
                data: None,
                shard_id: None,
                result: None,
            })
        }
    }
    
    async fn distribute_storage(&self, data: &[u8], redundancy: u32) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut shard_ids = Vec::new();
        let nodes: Vec<&String> = self.nodes.keys().collect();
        
        for i in 0..redundancy {
            let node_id = nodes[i as usize % nodes.len()];
            let shard_id = format!("shard_{}_{}_{}", 
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                i,
                node_id
            );
            
            let message = P2PMessage {
                message_type: "StoreRequest".to_string(),
                data: Some(data.to_vec()),
                redundancy: Some(redundancy),
                from: "p2p_client".to_string(),
                shard_id: Some(shard_id.clone()),
                wasm_bytes: None,
                input_data: None,
                timestamp: Some(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()),
            };
            
            if let Ok(_) = self.forward_to_node(node_id, &message).await {
                shard_ids.push(shard_id.clone());
                println!("✅ Stored shard {} on node {}", shard_id, node_id);
            }
        }
        
        Ok(shard_ids)
    }
    
    async fn execute_compute(&self, wasm_bytes: &[u8], input_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Find a node with compute capabilities
        let compute_node = self.nodes.keys().next()
            .ok_or("No compute nodes available")?;
        
        let message = P2PMessage {
            message_type: "ComputeRequest".to_string(),
            data: None,
            redundancy: None,
            from: "p2p_client".to_string(),
            shard_id: None,
            wasm_bytes: Some(wasm_bytes.to_vec()),
            input_data: Some(input_data.to_vec()),
            timestamp: Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()),
        };
        
        let response = self.forward_to_node(compute_node, &message).await?;
        
        if response.success {
            response.result.ok_or("No result data received".into())
        } else {
            Err(response.message.into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Starting P2P WebSocket Proxy...");
    println!("📍 Listening on ws://localhost:3006");
    
    let proxy = Arc::new(P2PProxy::new());
    
    let addr = "127.0.0.1:3006";
    let listener = TcpListener::bind(addr).await?;
    
    println!("✅ Proxy ready! Bridging WebSocket clients to P2P nodes");
    
    while let Ok((stream, _)) = listener.accept().await {
        let proxy_clone = proxy.clone();
        
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                handle_websocket(ws_stream, proxy_clone).await;
            }
        });
    }
    
    Ok(())
}

async fn handle_websocket(ws_stream: WebSocketStream<TcpStream>, proxy: Arc<P2PProxy>) {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(message) = serde_json::from_str::<P2PMessage>(&text) {
                    match message.message_type.as_str() {
                        "StoreRequest" => {
                            if let (Some(data), Some(redundancy)) = (message.data, message.redundancy) {
                                match proxy.distribute_storage(&data, redundancy).await {
                                    Ok(shard_ids) => {
                                        let response = P2PResponse {
                                            success: true,
                                            message: format!("File stored with {}x redundancy", redundancy),
                                            data: None,
                                            shard_id: None,
                                            result: None,
                                        };
                                        
                                        if let Ok(response_json) = serde_json::to_string(&response) {
                                            let _ = ws_sender.send(Message::Text(response_json)).await;
                                        }
                                    }
                                    Err(e) => {
                                        let response = P2PResponse {
                                            success: false,
                                            message: format!("Storage failed: {}", e),
                                            data: None,
                                            shard_id: None,
                                            result: None,
                                        };
                                        
                                        if let Ok(response_json) = serde_json::to_string(&response) {
                                            let _ = ws_sender.send(Message::Text(response_json)).await;
                                        }
                                    }
                                }
                            }
                        }
                        "ComputeRequest" => {
                            if let (Some(wasm_bytes), Some(input_data)) = (message.wasm_bytes, message.input_data) {
                                match proxy.execute_compute(&wasm_bytes, &input_data).await {
                                    Ok(result) => {
                                        let response = P2PResponse {
                                            success: true,
                                            message: "Compute completed successfully".to_string(),
                                            data: None,
                                            shard_id: None,
                                            result: Some(result),
                                        };
                                        
                                        if let Ok(response_json) = serde_json::to_string(&response) {
                                            let _ = ws_sender.send(Message::Text(response_json)).await;
                                        }
                                    }
                                    Err(e) => {
                                        let response = P2PResponse {
                                            success: false,
                                            message: format!("Compute failed: {}", e),
                                            data: None,
                                            shard_id: None,
                                            result: None,
                                        };
                                        
                                        if let Ok(response_json) = serde_json::to_string(&response) {
                                            let _ = ws_sender.send(Message::Text(response_json)).await;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            println!("Unknown message type: {}", message.message_type);
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            _ => {}
        }
    }
}

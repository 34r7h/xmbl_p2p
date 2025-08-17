use axum::{
    routing::{post, get, delete},
    http::{StatusCode, HeaderMap, HeaderValue},
    Json, Router,
    extract::{State, Path},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

// Import our actual Rust crates
use xmbl_storage::{StorageService, StorageShard};
use xmbl_network::{NetworkService, NetworkNode, NodeStatus};

#[derive(Clone)]
struct AppState {
    storage: Arc<Mutex<StorageService>>,
    network: Arc<Mutex<NetworkService>>,
}

#[derive(Deserialize)]
struct FileUploadRequest {
    filename: String,
    data: Vec<u8>,
    redundancy: u8,
}

#[derive(Serialize)]
struct FileUploadResponse {
    shard_id: String,
    checksum: String,
    nodes: Vec<String>,
    message: String,
}

#[derive(Serialize)]
struct StorageStatsResponse {
    used_gb: f64,
    total_gb: f64,
    shard_count: usize,
    available_nodes: usize,
}

#[derive(Serialize)]
struct FileInfo {
    shard_id: String,
    filename: String,
    size_bytes: usize,
    checksum: String,
    redundancy: u8,
    timestamp: String,
}

async fn upload_file(
    State(state): State<AppState>,
    Json(payload): Json<FileUploadRequest>,
) -> Result<Json<FileUploadResponse>, StatusCode> {
    let mut storage = state.storage.lock().await;
    let mut network = state.network.lock().await;
    
    // Store the file data using real storage service
    let shard_id = storage.store_data(payload.data.clone(), payload.redundancy)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Get the shard to get checksum
    let shard = storage.shards.get(&shard_id)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Get available nodes for redundancy
    let available_nodes: Vec<String> = network.nodes.values()
        .filter(|node| matches!(node.status, NodeStatus::Available))
        .map(|node| node.node_id.clone())
        .collect();
    
    Ok(Json(FileUploadResponse {
        shard_id,
        checksum: shard.checksum.clone(),
        nodes: available_nodes,
        message: format!("File stored successfully with {}x redundancy", payload.redundancy),
    }))
}

async fn get_storage_stats(
    State(state): State<AppState>,
) -> Result<Json<StorageStatsResponse>, StatusCode> {
    let storage = state.storage.lock().await;
    let network = state.network.lock().await;
    
    let (used_gb, total_gb) = storage.get_storage_stats();
    let shard_count = storage.shards.len();
    let available_nodes = network.nodes.values()
        .filter(|node| matches!(node.status, NodeStatus::Available))
        .count();
    
    Ok(Json(StorageStatsResponse {
        used_gb,
        total_gb,
        shard_count,
        available_nodes,
    }))
}

async fn list_files(
    State(state): State<AppState>,
) -> Result<Json<Vec<FileInfo>>, StatusCode> {
    let storage = state.storage.lock().await;
    
    let files: Vec<FileInfo> = storage.shards.values()
        .map(|shard| FileInfo {
            shard_id: shard.shard_id.clone(),
            filename: format!("file_{}", shard.shard_id[..8].to_string()),
            size_bytes: shard.data.len(),
            checksum: shard.checksum.clone(),
            redundancy: shard.redundancy,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
        .collect();
    
    Ok(Json(files))
}

async fn delete_file(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let shard_id = payload["shard_id"].as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let mut storage = state.storage.lock().await;
    
    storage.delete_data(shard_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({
        "message": "File deleted successfully",
        "shard_id": shard_id
    })))
}

async fn get_network_status(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let network = state.network.lock().await;
    
    let node_count = network.nodes.len();
    let online_nodes = network.nodes.values()
        .filter(|node| matches!(node.status, NodeStatus::Online))
        .count();
    let available_nodes = network.nodes.values()
        .filter(|node| matches!(node.status, NodeStatus::Available))
        .count();
    
    Ok(Json(serde_json::json!({
        "total_nodes": node_count,
        "online_nodes": online_nodes,
        "available_nodes": available_nodes,
        "network_status": if online_nodes > 0 { "healthy" } else { "offline" }
    })))
}

async fn download_file(
    State(state): State<AppState>,
    Path(shard_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let storage = state.storage.lock().await;
    
    // Try to retrieve the file data
    match storage.retrieve_data(&shard_id).await {
        Ok(data) => {
            // Create a response with the file data
            let filename = format!("file_{}", &shard_id[..8]);
            
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", HeaderValue::from_static("application/octet-stream"));
            headers.insert("Content-Disposition", HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).unwrap_or_else(|_| HeaderValue::from_static("attachment")));
            
            Ok((headers, data))
        }
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    // Initialize real storage service
    let storage = Arc::new(Mutex::new(StorageService::new(
        "web_api_node".to_string(),
        100.0 // 100GB storage
    )));
    
    // Initialize real network service
    let network = Arc::new(Mutex::new(NetworkService::new(
        "web_api_node".to_string()
    )));
    
    let state = AppState { storage, network };
    
    // Build our application with a route
    let app = Router::new()
        .route("/api/upload", post(upload_file))
        .route("/api/stats", get(get_storage_stats))
        .route("/api/files", get(list_files))
        .route("/api/files/:shard_id/download", get(download_file))
        .route("/api/files/delete", post(delete_file))
        .route("/api/network/status", get(get_network_status))
        .with_state(state);
    
    // Run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003").await.unwrap();
    println!("🚀 Real P2P Web API running on http://127.0.0.1:3003");
    println!("📁 Real storage service: ACTIVE");
    println!("🌐 Real network service: ACTIVE");
    
    axum::serve(listener, app).await.unwrap();
}

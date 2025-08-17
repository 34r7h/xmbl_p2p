use xmbl_monitoring::{MonitoringService, SystemMetrics, Alert, AlertSeverity, NetworkIO};
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
struct MetricsResponse {
    node_id: String,
    metrics: SystemMetrics,
    active_alerts: Vec<Alert>,
    status: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct AlertRequest {
    message: String,
    severity: String,
}

struct AppState {
    monitoring_service: Arc<Mutex<MonitoringService>>,
}

async fn get_metrics(State(state): State<Arc<AppState>>) -> Json<MetricsResponse> {
    let mut service = state.monitoring_service.lock().await;
    let metrics = service.collect_metrics().await.unwrap_or_else(|_| SystemMetrics {
        cpu_usage: 0.0,
        memory_usage: 0.0,
        disk_usage: 0.0,
        network_io: NetworkIO {
            bytes_in: 0,
            bytes_out: 0,
            packets_in: 0,
            packets_out: 0,
        },
        timestamp: 0,
        custom_metrics: HashMap::new(),
    });
    
    let active_alerts = service.get_active_alerts();
    
    Json(MetricsResponse {
        node_id: service.node_id.clone(),
        metrics,
        active_alerts: active_alerts.into_iter().cloned().collect(),
        status: "healthy".to_string(),
    })
}

async fn create_alert(
    State(state): State<Arc<AppState>>,
    Json(alert_req): Json<AlertRequest>,
) -> StatusCode {
    let mut service = state.monitoring_service.lock().await;
    
    let severity = match alert_req.severity.as_str() {
        "critical" => AlertSeverity::Critical,
        "warning" => AlertSeverity::Warning,
        "info" => AlertSeverity::Info,
        _ => AlertSeverity::Info,
    };
    
    let alert = Alert {
        alert_id: uuid::Uuid::new_v4().to_string(),
        message: alert_req.message,
        severity,
        timestamp: 0, // Using 0 as default timestamp
        acknowledged: false,
    };
    
    // Add alert to service
    service.alert_history.push(alert);
    
    StatusCode::CREATED
}

async fn get_alerts(State(state): State<Arc<AppState>>) -> Json<Vec<Alert>> {
    let service = state.monitoring_service.lock().await;
    Json(service.get_active_alerts().into_iter().cloned().collect())
}

async fn acknowledge_alert(
    State(state): State<Arc<AppState>>,
    Json(alert_id): Json<String>,
) -> StatusCode {
    let mut service = state.monitoring_service.lock().await;
    
    match service.acknowledge_alert(&alert_id) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

#[tokio::main]
async fn main() {
    // Initialize monitoring service
    let monitoring_service = MonitoringService::new("monitoring_node_001".to_string());
    let state = Arc::new(AppState {
        monitoring_service: Arc::new(Mutex::new(monitoring_service)),
    });

    // Build application
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .route("/alerts", get(get_alerts))
        .route("/alerts", post(create_alert))
        .route("/alerts/acknowledge", post(acknowledge_alert))
        .route("/health", get(|| async { "OK" }))
        .with_state(state);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 Monitoring service starting on {}", addr);
    println!("📊 Available endpoints:");
    println!("   GET  /health - Health check");
    println!("   GET  /metrics - Get system metrics");
    println!("   GET  /alerts - Get active alerts");
    println!("   POST /alerts - Create new alert");
    println!("   POST /alerts/acknowledge - Acknowledge alert");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

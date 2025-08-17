// XMBL Simulator Service - INDEPENDENT WITH MOCKS

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;
use uuid::Uuid;

// MOCK TYPES - All dependencies are mocked
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockNodeIdentity {
    pub node_id: String,
    pub public_key: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockNetworkService {
    pub node_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockStorageService {
    pub node_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockComputeService {
    pub node_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockBlockchainService {
    pub node_id: String,
}

// SIMULATOR TYPES
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulatedNode {
    pub node_id: String,
    pub identity: MockNodeIdentity,
    pub capabilities: MockNodeCapabilities,
    pub status: NodeStatus,
    pub performance: NodePerformance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockNodeCapabilities {
    pub storage_gb: f64,
    pub compute_flops: u64,
    pub bandwidth_mbps: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Busy,
    Available,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodePerformance {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub network_latency: f64,
    pub response_time: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationScenario {
    pub scenario_id: String,
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationResult {
    pub scenario_id: String,
    pub success: bool,
    pub metrics: HashMap<String, f64>,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}

pub struct SimulatorService {
    pub node_id: String,
    pub nodes: HashMap<String, SimulatedNode>,
    pub scenarios: HashMap<String, SimulationScenario>,
    pub results: HashMap<String, SimulationResult>,
}

impl SimulatorService {
    pub fn new(node_id: String) -> Self {
        SimulatorService {
            node_id,
            nodes: HashMap::new(),
            scenarios: HashMap::new(),
            results: HashMap::new(),
        }
    }
    
    pub async fn create_network(&mut self, node_count: usize) -> Result<()> {
        for i in 0..node_count {
            let node_id = format!("node_{}", i);
            let node = SimulatedNode {
                node_id: node_id.clone(),
                identity: MockNodeIdentity {
                    node_id: node_id.clone(),
                    public_key: vec![i as u8; 32],
                },
                capabilities: MockNodeCapabilities {
                    storage_gb: 1000.0 + (i as f64 * 100.0),
                    compute_flops: 1_000_000_000 + (i as u64 * 100_000_000),
                    bandwidth_mbps: 100.0 + (i as f64 * 10.0),
                },
                status: NodeStatus::Online,
                performance: NodePerformance {
                    cpu_usage: 0.1 + (i as f64 * 0.05),
                    memory_usage: 0.2 + (i as f64 * 0.03),
                    network_latency: 10.0 + (i as f64 * 2.0),
                    response_time: 50.0 + (i as f64 * 10.0),
                },
            };
            
            self.nodes.insert(node_id, node);
        }
        
        Ok(())
    }
    
    pub async fn run_scenario(&mut self, name: String, description: String, parameters: HashMap<String, String>) -> Result<String> {
        let scenario_id = Uuid::new_v4().to_string();
        let scenario = SimulationScenario {
            scenario_id: scenario_id.clone(),
            name,
            description,
            parameters,
        };
        
        self.scenarios.insert(scenario_id.clone(), scenario);
        
        // Simulate scenario execution
        let start_time = std::time::Instant::now();
        let success = self.execute_scenario(&scenario_id).await?;
        let duration = start_time.elapsed().as_millis() as u64;
        
        let result = SimulationResult {
            scenario_id: scenario_id.clone(),
            success,
            metrics: self.generate_metrics(),
            duration_ms: duration,
            error_message: None,
        };
        
        self.results.insert(scenario_id.clone(), result);
        Ok(scenario_id)
    }
    
    async fn execute_scenario(&self, scenario_id: &str) -> Result<bool> {
        // Simulate processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        
        // Mock success for now
        Ok(true)
    }
    
    fn generate_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("total_nodes".to_string(), self.nodes.len() as f64);
        metrics.insert("online_nodes".to_string(), self.nodes.values().filter(|n| matches!(n.status, NodeStatus::Online)).count() as f64);
        metrics.insert("avg_cpu_usage".to_string(), 0.25);
        metrics.insert("avg_memory_usage".to_string(), 0.45);
        metrics.insert("avg_network_latency".to_string(), 15.0);
        metrics
    }
    
    pub fn get_network_stats(&self) -> (usize, usize) {
        let total = self.nodes.len();
        let online = self.nodes.values().filter(|n| matches!(n.status, NodeStatus::Online)).count();
        (total, online)
    }
    
    pub fn get_scenario_results(&self, scenario_id: &str) -> Option<&SimulationResult> {
        self.results.get(scenario_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simulator_service_creation() {
        let service = SimulatorService::new("test_node".to_string());
        assert_eq!(service.node_id, "test_node");
    }

    #[tokio::test]
    async fn test_network_creation_and_scenario() {
        let mut service = SimulatorService::new("test_node".to_string());
        
        service.create_network(5).await.unwrap();
        let (total, online) = service.get_network_stats();
        assert_eq!(total, 5);
        assert_eq!(online, 5);
        
        let mut params = HashMap::new();
        params.insert("test_param".to_string(), "test_value".to_string());
        
        let scenario_id = service.run_scenario(
            "Test Scenario".to_string(),
            "A test scenario".to_string(),
            params
        ).await.unwrap();
        
        assert!(service.get_scenario_results(&scenario_id).is_some());
    }
}

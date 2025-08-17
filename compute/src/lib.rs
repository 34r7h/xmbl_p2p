// XMBL Compute Service - INDEPENDENT WITH MOCKS

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;
use uuid::Uuid;

// MOCK TYPES
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockNodeIdentity {
    pub node_id: String,
    pub public_key: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockNetworkService {
    pub node_id: String,
}

// REAL COMPUTE TYPES
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputeTask {
    pub task_id: String,
    pub wasm_bytes: Vec<u8>,
    pub input_data: Vec<u8>,
    pub task_type: TaskType,
    pub priority: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskType {
    WASM,
    MPC,
    Batch,
    RealTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub output_data: Vec<u8>,
    pub execution_time_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputeService {
    pub node_id: String,
    pub active_tasks: HashMap<String, ComputeTask>,
    pub completed_tasks: HashMap<String, TaskResult>,
    pub max_concurrent_tasks: usize,
    pub current_load: f64,
}

impl ComputeService {
    pub fn new(node_id: String, max_concurrent_tasks: usize) -> Self {
        ComputeService {
            node_id,
            active_tasks: HashMap::new(),
            completed_tasks: HashMap::new(),
            max_concurrent_tasks,
            current_load: 0.0,
        }
    }
    
    pub async fn submit_task(&mut self, wasm_bytes: Vec<u8>, input_data: Vec<u8>, task_type: TaskType) -> Result<String> {
        if self.active_tasks.len() >= self.max_concurrent_tasks {
            return Err(anyhow::anyhow!("Maximum concurrent tasks reached"));
        }
        
        let task_id = Uuid::new_v4().to_string();
        let task = ComputeTask {
            task_id: task_id.clone(),
            wasm_bytes,
            input_data,
            task_type,
            priority: 1,
        };
        
        self.active_tasks.insert(task_id.clone(), task);
        self.current_load = self.active_tasks.len() as f64 / self.max_concurrent_tasks as f64;
        
        Ok(task_id)
    }
    
    pub async fn execute_task(&mut self, task_id: &str) -> Result<TaskResult> {
        let task = self.active_tasks.get(task_id)
            .ok_or_else(|| anyhow::anyhow!("Task not found"))?;
        
        let start_time = std::time::Instant::now();
        
        // TODO: Implement actual WASM execution
        let output_data = self.simulate_wasm_execution(&task.wasm_bytes, &task.input_data).await?;
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        let result = TaskResult {
            task_id: task_id.to_string(),
            output_data,
            execution_time_ms: execution_time,
            success: true,
            error_message: None,
        };
        
        // Move task from active to completed
        self.active_tasks.remove(task_id);
        self.completed_tasks.insert(task_id.to_string(), result.clone());
        self.current_load = self.active_tasks.len() as f64 / self.max_concurrent_tasks as f64;
        
        Ok(result)
    }
    
    async fn simulate_wasm_execution(&self, wasm_bytes: &[u8], input_data: &[u8]) -> Result<Vec<u8>> {
        // Simulate processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Return mock output based on input size
        Ok(vec![0u8; input_data.len()])
    }
    
    pub fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        if self.active_tasks.contains_key(task_id) {
            Some(TaskStatus::Running)
        } else if self.completed_tasks.contains_key(task_id) {
            Some(TaskStatus::Completed)
        } else {
            None
        }
    }
    
    pub fn get_service_stats(&self) -> (usize, usize, f64) {
        (
            self.active_tasks.len(),
            self.completed_tasks.len(),
            self.current_load
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Running,
    Completed,
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compute_service_creation() {
        let service = ComputeService::new("test_node".to_string(), 10);
        assert_eq!(service.node_id, "test_node");
        assert_eq!(service.max_concurrent_tasks, 10);
    }

    #[tokio::test]
    async fn test_task_submission_and_execution() {
        let mut service = ComputeService::new("test_node".to_string(), 5);
        let wasm_bytes = b"mock_wasm_code".to_vec();
        let input_data = b"test_input".to_vec();
        
        let task_id = service.submit_task(wasm_bytes, input_data, TaskType::WASM).await.unwrap();
        let result = service.execute_task(&task_id).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.task_id, task_id);
    }

    #[tokio::test]
    async fn test_max_concurrent_tasks_error() {
        let mut service = ComputeService::new("test_node".to_string(), 1);
        let wasm_bytes = b"mock_wasm_code".to_vec();
        let input_data = b"test_input".to_vec();
        let _ = service.submit_task(wasm_bytes.clone(), input_data.clone(), TaskType::WASM).await.unwrap();
        let err = service.submit_task(wasm_bytes, input_data, TaskType::WASM).await;
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().to_string(), "Maximum concurrent tasks reached");
    }

    #[tokio::test]
    async fn test_execute_task_not_found_error() {
        let mut service = ComputeService::new("test_node".to_string(), 1);
        let err = service.execute_task("nonexistent_id").await;
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().to_string(), "Task not found");
    }

    #[tokio::test]
    async fn test_get_task_status_variants() {
        let mut service = ComputeService::new("test_node".to_string(), 2);
        let wasm_bytes = b"mock_wasm_code".to_vec();
        let input_data = b"test_input".to_vec();
        let task_id = service.submit_task(wasm_bytes, input_data, TaskType::WASM).await.unwrap();
        assert_eq!(service.get_task_status(&task_id), Some(TaskStatus::Running));
        let _ = service.execute_task(&task_id).await.unwrap();
        assert_eq!(service.get_task_status(&task_id), Some(TaskStatus::Completed));
        assert_eq!(service.get_task_status("bad_id"), None);
    }

    #[tokio::test]
    async fn test_get_service_stats() {
        let mut service = ComputeService::new("test_node".to_string(), 2);
        let wasm_bytes = b"mock_wasm_code".to_vec();
        let input_data = b"test_input".to_vec();
        let task_id = service.submit_task(wasm_bytes, input_data, TaskType::WASM).await.unwrap();
        let (active, completed, load) = service.get_service_stats();
        assert_eq!(active, 1);
        assert_eq!(completed, 0);
        assert!(load > 0.0);
        let _ = service.execute_task(&task_id).await.unwrap();
        let (active, completed, load) = service.get_service_stats();
        assert_eq!(active, 0);
        assert_eq!(completed, 1);
        assert_eq!(load, 0.0);
    }

    #[test]
    fn test_task_status_enum_variants() {
        let running = TaskStatus::Running;
        let completed = TaskStatus::Completed;
        let failed = TaskStatus::Failed;
        match running {
            TaskStatus::Running => {}
            _ => panic!("Should be running"),
        }
        match completed {
            TaskStatus::Completed => {}
            _ => panic!("Should be completed"),
        }
        match failed {
            TaskStatus::Failed => {}
            _ => panic!("Should be failed"),
        }
    }
}

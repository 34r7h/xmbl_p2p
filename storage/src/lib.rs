// XMBL Storage Service - INDEPENDENT WITH MOCKS

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

// REAL STORAGE TYPES
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageShard {
    pub shard_id: String,
    pub data: Vec<u8>,
    pub redundancy: u8,
    pub checksum: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageService {
    pub node_id: String,
    pub shards: HashMap<String, StorageShard>,
    pub total_storage_gb: f64,
    pub used_storage_gb: f64,
}

impl StorageService {
    pub fn new(node_id: String, total_storage_gb: f64) -> Self {
        StorageService {
            node_id,
            shards: HashMap::new(),
            total_storage_gb,
            used_storage_gb: 0.0,
        }
    }
    
    pub async fn store_data(&mut self, data: Vec<u8>, redundancy: u8) -> Result<String> {
        let shard_id = Uuid::new_v4().to_string();
        let checksum = self.calculate_checksum(&data);
        
        let shard = StorageShard {
            shard_id: shard_id.clone(),
            data,
            redundancy,
            checksum,
        };
        
        let data_size_gb = shard.data.len() as f64 / (1024.0 * 1024.0 * 1024.0);
        if self.used_storage_gb + data_size_gb > self.total_storage_gb {
            return Err(anyhow::anyhow!("Insufficient storage space"));
        }
        
        self.shards.insert(shard_id.clone(), shard);
        self.used_storage_gb += data_size_gb;
        
        Ok(shard_id)
    }
    
    pub async fn retrieve_data(&self, shard_id: &str) -> Result<Vec<u8>> {
        let shard = self.shards.get(shard_id)
            .ok_or_else(|| anyhow::anyhow!("Shard not found"))?;
        
        // Verify checksum
        let calculated_checksum = self.calculate_checksum(&shard.data);
        if calculated_checksum != shard.checksum {
            return Err(anyhow::anyhow!("Data corruption detected"));
        }
        
        Ok(shard.data.clone())
    }
    
    pub async fn delete_data(&mut self, shard_id: &str) -> Result<()> {
        if let Some(shard) = self.shards.remove(shard_id) {
            let data_size_gb = shard.data.len() as f64 / (1024.0 * 1024.0 * 1024.0);
            self.used_storage_gb -= data_size_gb;
        }
        Ok(())
    }
    
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
    
    pub fn get_storage_stats(&self) -> (f64, f64) {
        (self.used_storage_gb, self.total_storage_gb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_service_creation() {
        let service = StorageService::new("test_node".to_string(), 100.0);
        assert_eq!(service.node_id, "test_node");
        assert_eq!(service.total_storage_gb, 100.0);
    }

    #[tokio::test]
    async fn test_data_storage_and_retrieval() {
        let mut service = StorageService::new("test_node".to_string(), 100.0);
        let test_data = b"Hello, World!".to_vec();
        
        let shard_id = service.store_data(test_data.clone(), 3).await.unwrap();
        let retrieved_data = service.retrieve_data(&shard_id).await.unwrap();
        
        assert_eq!(test_data, retrieved_data);
    }
}

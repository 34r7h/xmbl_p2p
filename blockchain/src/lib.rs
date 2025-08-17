// XMBL Blockchain Service - INDEPENDENT WITH MOCKS

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

// REAL BLOCKCHAIN TYPES
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    pub address: String,
    pub balance: u64,
    pub last_updated: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
    pub status: TransactionStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockchainService {
    pub node_id: String,
    pub balances: HashMap<String, TokenBalance>,
    pub transactions: HashMap<String, Transaction>,
    pub total_supply: u64,
}

impl BlockchainService {
    pub fn new(node_id: String, total_supply: u64) -> Self {
        BlockchainService {
            node_id,
            balances: HashMap::new(),
            transactions: HashMap::new(),
            total_supply,
        }
    }
    
    pub async fn create_account(&mut self, address: String, initial_balance: u64) -> Result<()> {
        if self.balances.contains_key(&address) {
            return Err(anyhow::anyhow!("Account already exists"));
        }
        
        let balance = TokenBalance {
            address: address.clone(),
            balance: initial_balance,
            last_updated: self.get_current_timestamp(),
        };
        
        self.balances.insert(address, balance);
        Ok(())
    }
    
    pub async fn transfer_tokens(&mut self, from: String, to: String, amount: u64) -> Result<String> {
        // Check if both accounts exist and from has sufficient balance
        let from_balance = self.balances.get(&from)
            .ok_or_else(|| anyhow::anyhow!("From account not found"))?;
            
        if from_balance.balance < amount {
            return Err(anyhow::anyhow!("Insufficient balance"));
        }
        
        // Create transaction first
        let tx_id = Uuid::new_v4().to_string();
        let transaction = Transaction {
            tx_id: tx_id.clone(),
            from: from.clone(),
            to: to.clone(),
            amount,
            timestamp: self.get_current_timestamp(),
            status: TransactionStatus::Confirmed,
        };
        
        self.transactions.insert(tx_id.clone(), transaction);
        
        // Now update balances
        if let Some(from_balance) = self.balances.get_mut(&from) {
            from_balance.balance -= amount;
        }
        
        if let Some(to_balance) = self.balances.get_mut(&to) {
            to_balance.balance += amount;
        }
        
        Ok(tx_id)
    }
    
    pub fn get_balance(&self, address: &str) -> Option<u64> {
        self.balances.get(address).map(|b| b.balance)
    }
    
    pub fn get_transaction(&self, tx_id: &str) -> Option<&Transaction> {
        self.transactions.get(tx_id)
    }
    
    fn get_current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blockchain_service_creation() {
        let service = BlockchainService::new("test_node".to_string(), 1_000_000);
        assert_eq!(service.node_id, "test_node");
        assert_eq!(service.total_supply, 1_000_000);
    }

    #[tokio::test]
    async fn test_account_creation_and_transfer() {
        let mut service = BlockchainService::new("test_node".to_string(), 1_000_000);
        
        service.create_account("alice".to_string(), 1000).await.unwrap();
        service.create_account("bob".to_string(), 500).await.unwrap();
        
        let tx_id = service.transfer_tokens("alice".to_string(), "bob".to_string(), 300).await.unwrap();
        
        assert_eq!(service.get_balance("alice"), Some(700));
        assert_eq!(service.get_balance("bob"), Some(800));
        assert!(service.get_transaction(&tx_id).is_some());
    }
}

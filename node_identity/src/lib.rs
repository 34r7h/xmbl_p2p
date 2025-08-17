// XMBL Node Identity System
// INDEPENDENT DEVELOPMENT - No external dependencies

use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use hex;
use rand::rngs::OsRng;
use serde::Serialize;

#[derive(Serialize)]
pub struct NodeIdentity {
    pub node_id: String,
    pub public_key: PublicKey,
    pub token_balance: u64,
    #[serde(skip)]
    pub secret_key: SecretKey,
}

impl NodeIdentity {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        
        let pubkey_bytes = public_key.serialize_uncompressed();
        let mut hasher = Keccak256::new();
        hasher.update(&pubkey_bytes[1..]); // skip leading 0x04
        let result = hasher.finalize();
        let node_id = &result[result.len()-20..];
        
        NodeIdentity {
            node_id: format!("0x{}", hex::encode(node_id)),
            public_key,
            token_balance: 0,
            secret_key,
        }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn get_token_balance(&self) -> u64 {
        self.token_balance
    }

    pub fn update_token_balance(&mut self, new_balance: u64) {
        self.token_balance = new_balance;
    }
    
    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }
}

impl Default for NodeIdentity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_identity_creation() {
        let identity = NodeIdentity::new();
        assert!(!identity.node_id.is_empty());
        assert_eq!(identity.token_balance, 0);
    }

    #[test]
    fn test_token_balance_update() {
        let mut identity = NodeIdentity::new();
        identity.update_token_balance(1000);
        assert_eq!(identity.get_token_balance(), 1000);
    }
}

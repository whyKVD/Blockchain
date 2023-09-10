use crate::transaction::Transaction;
use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug,Clone)]
pub struct Block {
    pub index: i64,
    pub timestamp: i64,
    pub data: Vec<Transaction>,
    prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: i64, data: Vec<Transaction>, prev_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(index, timestamp, &data, &prev_hash);
        
        Block{
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: i64, timestamp: i64, data: &Vec<Transaction>, prev_hash: &str) -> String {
        format!("{:x}", Sha256::digest(format!("{}{}{:#?}{}", index, timestamp, data, prev_hash).as_bytes()))
    }
}

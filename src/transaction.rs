use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug,Clone)]
pub struct Transaction {
    pub from_addr: String,
    pub to_addr: String,
    pub amount: i64,
    pub timestamp: i64,
    pub hash: String,
}

impl Transaction {
    pub fn new(from_addr: String, to_addr: String, amount: i64) -> Transaction {
        let timestamp = Utc::now().timestamp();
        let hash = Transaction::calculate_hash(&from_addr,&to_addr,amount,timestamp);

        Transaction{
            from_addr,
            to_addr,
            amount,
            timestamp,
            hash,
        }
    }

    fn calculate_hash(from_addr: &str, to_addr: &str, amount: i64, timestamp: i64) -> String {
        format!("{:x}", Sha256::digest(format!("{}{}{}{}", from_addr, to_addr, amount, timestamp).as_bytes()))
    }
}

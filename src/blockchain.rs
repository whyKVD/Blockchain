use crate::{
    block::Block,
    transaction::Transaction,
    wallet::Wallet
};

#[derive(Debug)]
pub struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let admin = Wallet::new();
        let genesis_block = Block::new(0,vec![Transaction::new(String::new(),admin.public_key,1000)], "0".to_string());

        BlockChain{
            chain: vec![genesis_block],
        }
    }

    fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn add_block(&mut self, data: Vec<Transaction>) {
        self.chain.push(Block::new(self.chain.len() as i64, data, self.get_last_block().unwrap().hash.clone()));
    }

    pub fn is_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i > 0 {
                let prev_block = &self.chain[ i-1 ];
                if block.hash != Block::calculate_hash(
                    block.index,
                    block.timestamp,
                    &block.data,
                    &prev_block.hash,
                    ){
                    return false
                }
            }
        }

        true
    }

    pub fn get_balance(&self, addres: &str) -> i64 {
        let mut balance : i64 = 0;

        for block in self.chain.iter() {
            for transaction in block.data.iter() {
                if transaction.from_addr.eq(&addres) { balance -= transaction.amount};
                if transaction.to_addr.eq(&addres) { balance += transaction.amount};
            }
        }

        balance
    }
}

use sha2::{Sha256, Digest};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: i64,
    pub payload: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

pub struct Blockchain {
    chain: Vec<Block>
}

impl Block {
    pub fn new(id: u64, payload: String, previous_hash: String) -> Self {
        let now = SystemTime::now();

        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;

        let mut block = Block {
            id,
            timestamp,
            payload,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}{}",
            self.id,
            self.timestamp,
            self.payload,
            self.previous_hash,
            self.nonce
        );
        hasher.update(input);
        let result = hasher.finalize();

        let mut s = String::new();
        for byte in result {
            write!(&mut s, "{:02x}", byte).expect("Unable to write");
        }
        s
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let pattern = "0".repeat(difficulty);
        self.hash = self.calculate_hash();

        loop {
            if self.hash.starts_with(&pattern) {
                break;
            }
            
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { chain: Vec::new() };

        let id = 0;
        let payload = "First block".to_string();
        let previous_hash = "0".repeat(64);

        let mut first_block = Block::new(id, payload, previous_hash);
        first_block.mine_block(3);
        blockchain.chain.push(first_block);

        blockchain
    }

    pub fn add_block(&mut self, payload: String) {
        if let Some(last_block) = self.chain.last() {
            let id = last_block.id + 1;
            let previous_hash = last_block.hash.clone();

            let mut new_block = Block::new(id, payload, previous_hash);
            new_block.mine_block(3);

            self.chain.push(new_block);
        } else {
            println!("Chain is empty!");
        }
       
    }

    pub fn get_chain(&self) -> &[Block] {
        &self.chain
    }
}
use std::result;

use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

use super::blockchain::Blockchain;

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
   pub index: u64,
   pub timestamp: u64,

   pub data: String,
   pub proof_of_work: u64,
   pub previous_hash: String,
   pub hash: String
}
impl Block {
    pub fn new(index: u64, prev_hash: String,data: String) -> Self {
        let new_block = Block {
            index: index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            data: data,
            previous_hash: prev_hash,
            hash: String::default(),
        };
        new_block
    }


    pub fn mine(&mut self, blockchain: Blockchain) {
        /*
        select hash that's start with 0 and add int to proof_of_work
        */
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)){
                self.proof_of_work += 1;
                self.hash = self.generate_block_hash()
                
            } else {
                break;
            }
        }
    }

    pub fn generate_block_hash(&self) -> String {
        /*
        Block data -> get SHA256 hash of block data -> return in lowercase
        {:x} lowercase hex digits
        */
        let mut block_data = self.clone();
        block_data.hash = String::default();

        let serialized_block_data = 
            serde_json::to_string(&block_data).unwrap();
        
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);

        let result = hasher.finalize();
        format!("{:x}",result)

    }    
}
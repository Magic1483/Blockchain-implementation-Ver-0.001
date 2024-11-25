use super::block::Block;
use chrono::prelude::*;
// Internal module
use tabled::{Tabled,Table};

type Blocks = Vec<Block>;

// `Blockchain` A struct that represents the blockchain.
#[derive(Debug,Clone)]
pub struct Blockchain {
  // The first block to be added to the chain.
  pub genesis_block: Block,
  // The storage for blocks.
  pub chain: Blocks,
  // Minimum amount of work required to validate a block.
  pub difficulty: usize
}


impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
            data: String::from("genesis block"),
        };

        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty
        };
        blockchain // -> return 
    }

    pub fn check_data(&self){
        #[derive(Tabled)]
        struct row {
            index: u64,
            proof_of_work: u64,
            data: String,
            hash: String
        };

        let mut rows = Vec::new();

        for el in self.chain.clone() {
            rows.push(row{
                index: el.index,
                proof_of_work: el.proof_of_work,
                data: el.data,
                hash: el.hash
            })
        }

        let table = Table::new(rows).to_string();
        println!("{}",table);
    }

    pub fn add_block(&mut self,data: String) {
        let prev_hash = self.chain[&self.chain.len() - 1].hash.clone();
        let mut new_block = Block::new(
            self.chain.len() as u64,
            prev_hash,
            data
        );
        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        // println!("new block added to chain -> {} {:?}",new_block.index,new_block);

    }
}
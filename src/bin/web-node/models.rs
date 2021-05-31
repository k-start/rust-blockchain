use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RetBlock {
    pub index: u32,
    pub timestamp: u128,
    pub hash: String,
    pub prev_block_hash: String,
    pub nonce: u64,
    // pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

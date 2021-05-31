use super::{difficulty_bytes_as_u128, Hashable, Transaction};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block: {}, Hash: {}, Timestamp: {}, Nonce: {}, Difficulty: {}",
            self.index,
            hex::encode(&self.hash),
            self.timestamp,
            self.nonce,
            self.difficulty,
        )
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&(self.index).to_le_bytes());
        bytes.extend(&(self.timestamp).to_le_bytes());
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&(self.nonce).to_le_bytes());
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&(self.difficulty).to_le_bytes());

        bytes
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Vec<u8>,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Block {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();

            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }

    pub fn has_valid_transactions(&self) -> bool {
        for transaction in self.transactions.iter() {
            if !transaction.valid() {
                return false;
            }
        }
        true
    }
}

pub fn check_difficulty(hash: &Vec<u8>, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(hash)
}

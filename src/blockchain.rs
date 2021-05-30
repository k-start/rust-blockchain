use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub mining_reward: u64,
    pub difficulty: u128,
    unspent_outputs: HashSet<Vec<u8>>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: u128) -> Blockchain {
        Blockchain {
            blocks: vec![],
            mining_reward: 100,
            difficulty: difficulty,
            unspent_outputs: HashSet::new(),
            pending_transactions: vec![],
        }
    }

    pub fn create_genesis_block(&self) -> Block {
        Block::new(
            0,
            get_time(),
            vec![0; 32],
            vec![Transaction {
                inputs: vec![],
                outputs: vec![],
            }],
            self.difficulty,
        )
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(
        &mut self,
        reward_addr: &str,
    ) -> Result<(), BlockValidationErr> {
        let last_block = &self.blocks[self.blocks.len() - 1];

        // set up mining reward
        let mut transactions: Vec<Transaction> = vec![Transaction {
            inputs: vec![],
            outputs: vec![Output {
                to_addr: reward_addr.to_owned(),
                value: self.mining_reward,
            }],
        }];

        // add pending transactions
        transactions.extend(self.pending_transactions.clone());

        let mut block = Block::new(
            self.blocks.len() as u32,
            get_time(),
            last_block.hash.clone(),
            transactions,
            self.difficulty,
        );

        block.mine();
        println!("{:?}", &block);

        self.update_with_block(block)?;

        // reset pending transactions
        self.pending_transactions = vec![];

        Ok(())
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchIndex);
        }

        if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        }

        if i != 0 {
            let prev_block = &self.blocks[i - 1];

            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            }

            if block.prev_block_hash != prev_block.hash() {
                return Err(BlockValidationErr::MismatchPreviousHash);
            }
        } else {
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Vec<u8>> = HashSet::new();
            let mut block_created: HashSet<Vec<u8>> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));

            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }

    pub fn get_balance(&self, addr: &str) -> u64 {
        let mut balance = 0;

        for block in self.blocks.iter() {
            for transaction in block.transactions.iter() {
                for input in transaction.inputs.iter() {
                    if input.to_addr == addr {
                        balance -= input.value;
                    }
                }

                for output in transaction.outputs.iter() {
                    if output.to_addr == addr {
                        balance += output.value;
                    }
                }
            }
        }

        balance
    }
}

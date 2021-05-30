extern crate hex;

mod block;
mod blockchain;
mod hashable;
mod transaction;

use block::Block;
use blockchain::Blockchain;
use hashable::Hashable;
use transaction::{Output, Transaction};

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let difficulty: u128 = 0x000fffffffffffffffffffffffffffff;

    let mut blockchain = Blockchain::new(difficulty);

    let mut genesis_block = Blockchain::create_genesis_block(difficulty);

    genesis_block.mine();
    println!("{:?}", &genesis_block);

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    blockchain
        .mine_pending_transactions("bob")
        .expect("Failed to add block");

    blockchain.create_transaction(Transaction {
        inputs: vec![Output {
            to_addr: "bob".to_owned(),
            value: 100,
        }],
        outputs: vec![
            Output {
                to_addr: "bob".to_owned(),
                value: 50,
            },
            Output {
                to_addr: "alice".to_owned(),
                value: 50,
            },
        ],
    });

    blockchain
        .mine_pending_transactions("john")
        .expect("Failed to add block");
}

pub fn difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << 0xf * 8)
        | ((v[30] as u128) << 0xe * 8)
        | ((v[29] as u128) << 0xd * 8)
        | ((v[28] as u128) << 0xc * 8)
        | ((v[27] as u128) << 0xb * 8)
        | ((v[26] as u128) << 0xa * 8)
        | ((v[25] as u128) << 0x9 * 8)
        | ((v[24] as u128) << 0x8 * 8)
        | ((v[23] as u128) << 0x7 * 8)
        | ((v[22] as u128) << 0x6 * 8)
        | ((v[21] as u128) << 0x5 * 8)
        | ((v[20] as u128) << 0x4 * 8)
        | ((v[19] as u128) << 0x3 * 8)
        | ((v[18] as u128) << 0x2 * 8)
        | ((v[17] as u128) << 0x1 * 8)
        | ((v[16] as u128) << 0x0 * 8)
}

pub fn get_time() -> u128 {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    in_ms as u128
}

extern crate hex;
extern crate ring;

mod block;
mod blockchain;
mod hashable;
mod transaction;

use block::Block;
use blockchain::Blockchain;
use hashable::Hashable;
use transaction::{Output, Transaction};

use ring::{
    rand,
    signature::{self, KeyPair},
};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut blockchain = Blockchain::new(0x000fffffffffffffffffffffffffffff);

    let mut genesis_block = blockchain.create_genesis_block();

    genesis_block.mine();
    println!("{:?}", &genesis_block);

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    blockchain
        .mine_pending_transactions(
            "eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0",
        )
        .expect("Failed to add block");

    let mut transaction = Transaction::new(
        vec![Output {
            to_addr: "eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0".to_owned(),
            value: 100,
        }],
        vec![
            Output {
                to_addr: "eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0"
                    .to_owned(),
                value: 50,
            },
            Output {
                to_addr: "6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a"
                    .to_owned(),
                value: 50,
            },
        ],
    );
    transaction.sign("3053020101300506032b657004220420bd0450ea54ef1add8d34d5cdbcbbe05afdfba6c54a5cd4d159b0d707a3b0d45ca123032100eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0");
    blockchain.add_transaction(transaction);

    blockchain
        .mine_pending_transactions(
            "6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a",
        )
        .expect("Failed to add block");

    println!(
        "eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0: {}",
        blockchain.get_balance("eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0")
    );
    println!(
        "6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a: {}",
        blockchain.get_balance("6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a")
    );
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

pub fn generate_keys() {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

    let key_pair_hex = hex::encode(pkcs8_bytes.as_ref());
    let public_key_hex = hex::encode(key_pair.public_key().as_ref());
    let private_key_hex = key_pair_hex.replace(&public_key_hex, "");

    println!("Key Pair: {}", key_pair_hex);
    println!("Public key: {}", public_key_hex);
    println!("Private key: {}", private_key_hex);

    let key_pair_2 = signature::Ed25519KeyPair::from_pkcs8(
        &hex::decode(private_key_hex + &public_key_hex).unwrap(),
    )
    .unwrap();

    assert_eq!(
        key_pair_2.public_key().as_ref(),
        key_pair.public_key().as_ref()
    );
}

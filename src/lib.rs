pub mod block;
pub mod blockchain;
mod hashable;
pub mod transaction;

pub use crate::block::Block;
pub use crate::blockchain::Blockchain;
use crate::hashable::Hashable;
pub use crate::transaction::{Output, Transaction};

use ring::{
    rand,
    signature::{self, KeyPair},
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> u128 {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    in_ms as u128
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

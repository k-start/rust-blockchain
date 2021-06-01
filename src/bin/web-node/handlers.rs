use super::Chain;

use crate::models::{Balance, RetBlock};

pub async fn get_block(chain: Chain) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = chain.lock().await;

    let block = blockchain.blocks[0].clone();

    let ret_block = RetBlock {
        index: block.index,
        timestamp: block.timestamp,
        hash: hex::encode(block.hash),
        prev_block_hash: hex::encode(block.prev_block_hash),
        nonce: block.nonce,
        difficulty: block.difficulty,
    };
    return Ok(warp::reply::json(&ret_block));
}

pub async fn get_balance(addr: String, chain: Chain) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = chain.lock().await;

    let balance = blockchain.get_balance(&addr);

    let ret = Balance { balance: balance };

    return Ok(warp::reply::json(&ret));
}

pub async fn mine(addr: String, chain: Chain) -> Result<impl warp::Reply, warp::Rejection> {
    let mut blockchain = chain.lock().await;

    blockchain.mine_pending_transactions(&addr).expect("yotte");

    let balance = blockchain.get_balance(&addr);
    let ret = Balance { balance: balance };

    return Ok(warp::reply::json(&ret));
}

mod handlers;
mod models;
mod routes;

use blockchainlib::Blockchain;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Chain = Arc<Mutex<Blockchain>>;

#[tokio::main]
async fn main() {
    let chain = init_blockchain();
    let path = routes::routes(chain);

    warp::serve(path).run(([0, 0, 0, 0], 3030)).await;
}

fn init_blockchain() -> Chain {
    let mut blockchain = Blockchain::new(0x000fffffffffffffffffffffffffffff);
    let mut genesis_block = blockchain.create_genesis_block();
    genesis_block.mine();
    blockchain
        .update_with_block(genesis_block.clone())
        .expect("Failed to add genesis block");
    println!("{:?}", &genesis_block);

    Arc::new(Mutex::new(blockchain))
}

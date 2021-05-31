use blockchainlib::{Blockchain, Output, Transaction};

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

    println!(
        "eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0: {}",
        blockchain.get_balance("eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0")
    );
    println!(
        "6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a: {}",
        blockchain.get_balance("6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a")
    );

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

    let mut transaction2 = Transaction::new(
        vec![Output {
            to_addr: "6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a".to_owned(),
            value: 100,
        }],
        vec![
            Output {
                to_addr: "eec581be559c653d7ffa92a38cb5b1b13921e0ea3d4dd727c3737390b4d3caf0"
                    .to_owned(),
                value: 25,
            },
            Output {
                to_addr: "6abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a"
                    .to_owned(),
                value: 75,
            },
        ],
    );
    transaction2.sign("3053020101300506032b657004220420cda7b04287faffb55c8132aa7c9a1d47ea8f9c1dd0f14c224f51720d71c67b88a1230321006abfd8aea2936793cf810de8ea2fd09713daaef79fa84103d48b547ae89c1f2a");
    blockchain.add_transaction(transaction2);

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

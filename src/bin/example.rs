use blockchainlib::{generate_keys, Blockchain, Output, Transaction};

fn main() {
    generate_keys();
    let mut blockchain = Blockchain::new(0x000fffffffffffffffffffffffffffff);

    let mut genesis_block = blockchain.create_genesis_block();
    genesis_block.mine();
    println!("{:?}", &genesis_block);

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    blockchain
        .mine_pending_transactions("5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM")
        .expect("Failed to add block");

    println!(
        "5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM: {}",
        blockchain.get_balance("5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM")
    );
    println!(
        "HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1: {}",
        blockchain.get_balance("HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1")
    );

    let mut transaction = Transaction::new(
        vec![Output {
            to_addr: "5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM".to_owned(),
            value: 100,
        }],
        vec![
            Output {
                to_addr: "HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1".to_owned(),
                value: 50,
            },
            Output {
                to_addr: "5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM".to_owned(),
                value: 50,
            },
        ],
    );
    transaction.sign("GD8M1Qm17WXoukx8QqqfvYvRCSzZH61yruT8qT2dTsctMqTkiLRB64nSfC9UfBuSXcRyTMWsR9k12KAUfkaHxrkALnUagWNhHMQXyCV6zeQsiYXCj1n1");
    blockchain.add_transaction(transaction);

    blockchain
        .mine_pending_transactions("HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1")
        .expect("Failed to add block");

    println!(
        "5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM: {}",
        blockchain.get_balance("5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM")
    );
    println!(
        "HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1: {}",
        blockchain.get_balance("HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1")
    );

    let mut transaction2 = Transaction::new(
        vec![Output {
            to_addr: "HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1".to_owned(),
            value: 100,
        }],
        vec![
            Output {
                to_addr: "5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM".to_owned(),
                value: 25,
            },
            Output {
                to_addr: "HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1".to_owned(),
                value: 75,
            },
        ],
    );
    transaction2.sign("GD8M1Qm17WXoukx8QqqfvYFsfG6ZXiETAztk8j5odyzEHXBPXdMuWABSx5xSKLzDCHRZNKvUNHfiXozEc7yguDsTNsux7stNCqwdTqxQhcnztN8qKvWj");
    blockchain.add_transaction(transaction2);

    blockchain
        .mine_pending_transactions("HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1")
        .expect("Failed to add block");

    println!(
        "5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM: {}",
        blockchain.get_balance("5vXrCcVkxSvtmJX3dbkbdfnbGFBieeB1XbQWkPAL42EM")
    );
    println!(
        "HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1: {}",
        blockchain.get_balance("HTXECUvaxPftxegQeoVD4fvpSJf9zqjJxLkN7H3GBif1")
    );
}

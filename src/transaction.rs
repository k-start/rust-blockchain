use super::{get_time, Hashable};
use ring::signature::{self, KeyPair};
use std::collections::HashSet;

type Address = String;

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&(self.value).to_le_bytes());

        bytes
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
    pub timestamp: u128,
    pub signature: Vec<u8>,
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes.extend(&(self.timestamp).to_le_bytes());

        bytes
    }
}

impl Transaction {
    pub fn new(inputs: Vec<Output>, outputs: Vec<Output>) -> Transaction {
        Transaction {
            inputs: inputs,
            outputs: outputs,
            timestamp: get_time(),
            signature: vec![],
        }
    }

    pub fn sign(&mut self, key_pair_str: &str) {
        let key_pair =
            signature::Ed25519KeyPair::from_pkcs8(&bs58::decode(key_pair_str).into_vec().unwrap())
                .unwrap();

        if self.inputs[0].to_addr != bs58::encode(key_pair.public_key().as_ref()).into_string() {
            panic!("Cannot sign this transaction");
        }

        let transaction_hash = self.hash();
        let signature = key_pair.sign(&transaction_hash);
        self.signature = signature.as_ref().to_vec();
    }

    pub fn valid(&self) -> bool {
        if self.inputs.len() == 0 {
            return true;
        }

        if self.signature.len() == 0 {
            panic!("No signature");
        }

        let public_key = bs58::decode(self.inputs[0].to_addr.clone())
            .into_vec()
            .unwrap();
        let peer_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, &public_key);
        peer_public_key
            .verify(&self.hash(), &self.signature)
            .unwrap();

        true
    }

    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Vec<u8>> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Vec<u8>>>()
    }

    pub fn output_hashes(&self) -> HashSet<Vec<u8>> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Vec<u8>>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

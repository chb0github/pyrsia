/*
   Copyright 2021 JFrog Ltd

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use libp2p::core::identity::PublicKey::Ed25519;
use libp2p::identity::ed25519::Keypair;

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use super::header::{Address, Header};
use super::transaction::Transaction;
use crate::blockchain::BlockKeypair;
use crate::crypto::hash_algorithm::HashDigest;
use crate::signature::Signature;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
    pub signing_key: BlockKeypair,
    pub signature: Signature,
}
impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.header.ordinal.cmp(&other.header.ordinal)
    }
}

impl Block {
    pub fn new(
        parent_hash: HashDigest,
        ordinal: u128,
        transactions: Vec<Transaction>,
        signing_key: &BlockKeypair,
    ) -> Self {
        let transaction_root = HashDigest::new(&bincode::serialize(&transactions).unwrap());
        let header = Header::new(
            parent_hash,
            transaction_root,
            Address::from(Ed25519(signing_key.public())),
            ordinal,
        );
        let msg: Vec<u8> = format_header(&header);
        Self {
            header,
            transactions,
            signing_key: signing_key.clone(),
            signature: Signature::new(&msg, &signing_key),
        }
    }

    pub fn id(&self) -> String {
        self.header.hash().as_string()
    }
    pub fn ordinal(&self) -> u128 {
        self.header.ordinal as u128
    }
    pub fn signature(&self) -> String {
        self.signature.as_string()
    }

    // After merging Aleph consensus algorithm, it would be implemented
    pub fn verify(&self) -> Result<(), &str> {
        let msg: Vec<u8> = format_header(&self.header);
        if self.signature.verify(&msg, &self.signing_key) {
            return Ok(())
        }
        Err("Signature verification fail")
    }
}

fn format_header(header: &Header) -> Vec<u8> {
    bincode::serialize(&header.hash()).unwrap()
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.header.ordinal.partial_cmp(&other.header.ordinal)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(&self).expect("json format error");
        write!(f, "{}", json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_build_block() {
        use libp2p::identity;

        let keypair = identity::ed25519::Keypair::generate();
        let local_id = Address::from(Ed25519(keypair.public()));

        let transactions = vec![Transaction::new(
            local_id,
            json!("Hello First Transaction"),
            &BlockKeypair::new(&keypair),
        )];
        let block = Block::new(
            HashDigest::new(b""),
            1,
            transactions.to_vec(),
            &BlockKeypair::new(&keypair),
        );
        let signature = keypair.sign(&bincode::serialize(&block.header.hash()).unwrap());
        let expected_signature = Signature::new(
            &bincode::serialize(&block.header.hash()).unwrap(),
            &BlockKeypair::new(&keypair),
        );

        assert_eq!(1, block.header.ordinal);
        assert_eq!(expected_signature.as_string(), block.signature());
        assert!(keypair.public().verify(
            &bincode::serialize(&block.header.hash()).unwrap(),
            &signature
        ));
        block.verify().unwrap();
    }
}

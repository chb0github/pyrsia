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

use identity::ed25519::Keypair;
use libp2p::identity;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use super::header::Address;
use crate::blockchain::BlockKeypair;
use crate::crypto::hash_algorithm::HashDigest;
use crate::signature::Signature;

// Temporary structure to be able to calculate the hash of a transaction
#[derive(Serialize)]
struct PartialTransaction {
    submitter: Address,
    timestamp: u64,
    payload: Value,
    nonce: u128,
}

impl PartialTransaction {
    fn convert_to_transaction(
        self,
        ed25519_keypair: &BlockKeypair,
    ) -> Result<Transaction, bincode::Error> {
        let hash = calculate_hash(&self)?;
        Ok(Transaction {
            submitter: self.submitter,
            timestamp: self.timestamp,
            payload: self.payload,
            nonce: self.nonce,
            hash,
            signature: Signature::new(&bincode::serialize(&hash)?, ed25519_keypair),
        })
    }
}

impl From<Transaction> for PartialTransaction {
    fn from(transaction: Transaction) -> Self {
        PartialTransaction {
            submitter: transaction.submitter,
            timestamp: transaction.timestamp,
            payload: transaction.payload,
            nonce: transaction.nonce,
        }
    }
}

fn calculate_hash(
    incomplete_transaction: &PartialTransaction,
) -> Result<HashDigest, bincode::Error> {
    let bytes = bincode::serialize(incomplete_transaction)?;
    Ok(HashDigest::new(&bytes))
}

pub type TransactionSignature = Signature;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    submitter: Address,
    timestamp: u64,
    payload: Value,
    nonce: u128,
    // Adds a salt to harden
    hash: HashDigest,
    signature: TransactionSignature,
}

impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.submitter.hash(state);
        self.timestamp.hash(state);
        hash_value(&self.payload, state);
        self.nonce.hash(state);
        self.hash.hash(state);
        self.signature.hash(state);
    }
}

impl Transaction {
    pub fn new(submitter: Address, payload: Value, ed25519_keypair: &BlockKeypair) -> Self {
        let partial_transaction = PartialTransaction {
            submitter,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            payload,
            nonce: rand::thread_rng().gen::<u128>(),
        };
        partial_transaction
            .convert_to_transaction(ed25519_keypair)
            .unwrap()
    }

    pub fn digest(&self) -> HashDigest {
        self.hash
    }
    pub fn payload(&self) -> Value {
        self.payload.clone()
    }
    pub fn signature(&self) -> TransactionSignature {
        self.signature.clone()
    }
}

fn hash_value<H: Hasher>(val: &Value, state: &mut H) {
    match val {
        Value::Null => 0.hash(state),
        Value::Bool(b) => b.hash(state),
        Value::Number(n) => n.hash(state),
        Value::String(s) => s.hash(state),
        Value::Array(a) => a.iter().for_each(|v| hash_value(v, state)),
        Value::Object(o) => o.iter().for_each(|e| {
            e.0.hash(state);
            hash_value(e.1, state);
        }),
    };
    ()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_transaction_new() {
        let keypair = Keypair::generate();
        let local_id = Address::from(identity::PublicKey::Ed25519(keypair.public()));

        let transaction = Transaction::new(
            local_id,
            json!("Hello First Transaction"),
            &BlockKeypair::new(&keypair),
        );
        let partial: PartialTransaction = transaction.clone().into();
        let expected_hash = calculate_hash(&partial).unwrap();
        let expected_signature = Signature::new(
            &bincode::serialize(&expected_hash).unwrap(),
            &BlockKeypair::new(&keypair),
        );

        assert_eq!(expected_hash, transaction.digest());
        assert_eq!(expected_signature, transaction.signature());
    }
}

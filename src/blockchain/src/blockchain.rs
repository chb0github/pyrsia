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
use crate::crypto::hash_algorithm::HashDigest;
use identity::PublicKey::Ed25519;
use libp2p::identity;
use libp2p::identity::ed25519::Keypair;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::DeserializeFromStr;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::io::*;
use std::str::FromStr;
use std::{fs, io};

use super::structures::{block::Block, chain::Chain, header::Address, transaction::Transaction};

#[derive(serde_with::DeserializeFromStr)]
pub struct BlockKeypair(libp2p::identity::ed25519::Keypair);

impl BlockKeypair {
    pub fn public(&self) -> libp2p::identity::ed25519::PublicKey {
        self.0.public()
    }
    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.0.sign(msg)
    }
    pub fn verify(&self, msg: &Vec<u8>, signature: &Vec<u8>) -> bool {
        self.0.public().verify(msg, signature)
    }
    pub fn new(keypair: &libp2p::core::identity::ed25519::Keypair) -> Self {
        BlockKeypair(keypair.clone())
    }
}
impl std::fmt::Debug for BlockKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockKeypair")
            .field("keypair", &self.0)
            .finish()
    }
}

impl Serialize for BlockKeypair {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("")
    }
}
impl FromStr for BlockKeypair {
    type Err = String;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(BlockKeypair{0: Keypair::generate()})
    }
}

impl std::hash::Hash for BlockKeypair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.encode().hash(state)
    }
}
impl PartialEq for BlockKeypair {
    fn eq(&self, other: &Self) -> bool {
        self.0.encode().eq(&other.0.encode())
    }
}
impl Eq for BlockKeypair {}
impl Clone for BlockKeypair {
    fn clone(&self) -> Self {
        BlockKeypair(self.0.clone())
    }
}

const GENESIS_BLOCK: &str = r#"
{
  "header": {
    "parent_hash": {
      "multihash": {
        "code": 27,
        "size": 32,
        "digest": [
          197,
          210,
          70,
          1,
          134,
          247,
          35,
          60,
          146,
          126,
          125,
          178,
          220,
          199,
          3,
          192,
          229,
          0,
          182,
          83,
          202,
          130,
          39,
          59,
          123,
          250,
          216,
          4,
          93,
          133,
          164,
          112,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0
        ]
      }
    },
    "transactions_hash": {
      "multihash": {
        "code": 27,
        "size": 32,
        "digest": [
          233,
          108,
          53,
          79,
          93,
          137,
          203,
          205,
          252,
          249,
          183,
          146,
          181,
          241,
          197,
          237,
          208,
          38,
          159,
          220,
          217,
          66,
          20,
          136,
          171,
          18,
          2,
          155,
          138,
          230,
          87,
          120,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0
        ]
      }
    },
    "committer": {
      "peer_id": {
        "code": 0,
        "size": 36,
        "digest": [
          8,
          1,
          18,
          32,
          88,
          88,
          21,
          196,
          249,
          159,
          23,
          207,
          76,
          169,
          83,
          37,
          65,
          110,
          39,
          190,
          211,
          20,
          9,
          200,
          227,
          133,
          170,
          74,
          15,
          143,
          73,
          34,
          109,
          143,
          236,
          112,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0
        ]
      }
    },
    "timestamp": 1652285632,
    "ordinal": 0,
    "nonce": 303860116025340289318370235156069422466,
    "hash": {
      "multihash": {
        "code": 27,
        "size": 32,
        "digest": [
          127,
          122,
          125,
          245,
          17,
          128,
          125,
          0,
          44,
          30,
          218,
          171,
          209,
          181,
          122,
          156,
          224,
          49,
          61,
          237,
          138,
          196,
          246,
          39,
          243,
          253,
          60,
          169,
          205,
          249,
          127,
          22,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0,
          0
        ]
      }
    }
  },
  "transactions": [
    {
      "type_id": "AddAuthority",
      "submitter": {
        "peer_id": {
          "code": 0,
          "size": 36,
          "digest": [
            8,
            1,
            18,
            32,
            88,
            88,
            21,
            196,
            249,
            159,
            23,
            207,
            76,
            169,
            83,
            37,
            65,
            110,
            39,
            190,
            211,
            20,
            9,
            200,
            227,
            133,
            170,
            74,
            15,
            143,
            73,
            34,
            109,
            143,
            236,
            112,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ]
        }
      },
      "timestamp": 1652285632,
      "payload": [
        88,
        88,
        21,
        196,
        249,
        159,
        23,
        207,
        76,
        169,
        83,
        37,
        65,
        110,
        39,
        190,
        211,
        20,
        9,
        200,
        227,
        133,
        170,
        74,
        15,
        143,
        73,
        34,
        109,
        143,
        236,
        112
      ],
      "nonce": 227996946057909480709948369727184084395,
      "hash": {
        "multihash": {
          "code": 27,
          "size": 32,
          "digest": [
            191,
            136,
            186,
            142,
            71,
            35,
            50,
            249,
            32,
            160,
            230,
            17,
            206,
            183,
            79,
            164,
            28,
            174,
            40,
            51,
            236,
            111,
            110,
            162,
            24,
            63,
            149,
            74,
            47,
            141,
            78,
            100,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0
          ]
        }
      },
      "signature": {
        "signature": [
          125,
          122,
          129,
          45,
          96,
          186,
          139,
          203,
          117,
          130,
          19,
          25,
          184,
          27,
          154,
          163,
          204,
          133,
          72,
          25,
          254,
          104,
          74,
          198,
          249,
          196,
          247,
          96,
          104,
          197,
          222,
          90,
          161,
          134,
          116,
          112,
          199,
          195,
          66,
          114,
          224,
          224,
          71,
          67,
          37,
          50,
          18,
          193,
          218,
          113,
          219,
          3,
          64,
          166,
          112,
          174,
          30,
          238,
          207,
          94,
          84,
          158,
          71,
          9
        ]
      }
    }
  ],
  "signature": {
    "signature": [
      172,
      60,
      6,
      69,
      205,
      150,
      38,
      156,
      243,
      90,
      183,
      137,
      240,
      110,
      44,
      245,
      168,
      66,
      96,
      126,
      168,
      189,
      116,
      109,
      74,
      69,
      147,
      246,
      251,
      121,
      163,
      195,
      77,
      240,
      125,
      3,
      74,
      18,
      10,
      25,
      152,
      92,
      183,
      132,
      119,
      174,
      104,
      36,
      111,
      108,
      188,
      100,
      16,
      213,
      27,
      176,
      101,
      110,
      65,
      223,
      69,
      201,
      139,
      15
    ]
  }
}
"#;

/// Define Supported Signature Algorithm
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignatureAlgorithm {
    Ed25519,
}

pub struct Blockchain {
    // this should actually be a Map<Transaction,Vec<OnTransactionSettled>> but that's later
    trans_observers: HashMap<Transaction, Box<dyn FnOnce(Transaction)>>,
    block_observers: Vec<Box<dyn FnMut(Block)>>,
    pending_transaction: HashSet<Transaction>,
    keypair: BlockKeypair,
    submitter: Address,
    chain: Chain,
}

impl Debug for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blockchain")
            .field("chain", &self.chain)
            .field("submitter", &self.submitter)
            .field("trans_observers", &self.trans_observers.len())
            .field("block_observers", &self.block_observers.len())
            .finish()
    }
}

impl Blockchain {
    pub fn new(keypair: &BlockKeypair) -> Self {
        let submitter = Address::from(Ed25519(keypair.public()));
        let genesis_block: Block = serde_json::from_str(GENESIS_BLOCK).expect("");
        let mut chain: Chain = Default::default();
        chain.blocks.push(genesis_block);

        let mut me = Blockchain {
            trans_observers: Default::default(),
            block_observers: vec![],
            pending_transaction: Default::default(),
            keypair: keypair.clone(),
            submitter,
            chain,
        };
        me.add_block_listener(move |b: Block| {
            write_block(&b).expect("Block written to disk");
        });
        me
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.chain.blocks.clone()
    }
    pub fn save(&mut self) {
        let txs = self.pending_transaction.drain().collect();
        let last = self.chain.blocks.last().unwrap().clone();
        self.add_block(Block::new(
            last.header.hash(),
            last.ordinal() + 1,
            txs,
            &self.keypair,
        ))
    }
    /// When submitting a transaction, it may not settle for some time as it will be settled
    /// With other transactions as a block when this node is selected as the authority.
    /// The following are working examples of how to use this API
    /// *Example*
    /// ```rust
    /// use std::collections::HashMap;
    /// use serde::Serialize;
    /// use pyrsia_blockchain_network::blockchain::{Blockchain, create_ed25519_keypair};
    /// #[derive(Serialize)]
    /// struct Thing {
    ///     name: String,
    ///     age: usize,
    /// }
    /// let thing = Thing {
    ///     name: String::from("Christian Bongiorno"),
    ///     age: 10
    /// };
    ///  let keypair = create_ed25519_keypair("keypair");
    ///  let mut bc = Blockchain::new(&keypair);
    ///  bc.submit_transaction(thing, |t| {
    ///     println!("transaction  accepted {}", t.signature().as_string());
    ///  });
    ///  bc.submit_transaction([1, 2, 3], |t| {
    ///    println!("transaction  accepted {}", t.signature().as_string());
    ///  });
    ///  let map = HashMap::from([
    ///     ("im-a-map", String::from("hello")),
    ///     ("something", String::from("10")),
    /// ]);
    /// bc.submit_transaction(map, |t| {
    ///     println!("transaction  accepted {}", t.signature().as_string());
    /// });
    /// ```
    /// There are some caveats here
    /// 1. Usage of a map, with mixed types, is not possible.
    /// A Java equivalent of `Map<String,Object>` doesn't see to be doable. So you can't have
    /// ```rust
    /// use std::collections::HashMap;
    /// let map = HashMap::from([
    ///     ("im-a-map", String::from("hello")),
    ///    // ("something", 10), // won't compile
    /// ]);
    /// ```
    /// Because the Map derives it's generic types from the first tuple, which is different from the second
    ///
    pub fn submit_transaction<T, CallBack: 'static + FnOnce(Transaction)>(
        &mut self,
        payload: T,
        on_done: CallBack,
    ) -> Transaction
    where
        T: Sized + Serialize,
    {
        let trans = Transaction::new(self.submitter, json!(payload), &self.keypair);

        self.trans_observers
            .insert(trans.clone(), Box::new(on_done));
        self.pending_transaction.insert(trans.clone());
        trans.clone()
    }

    pub fn notify_transaction_settled(&mut self, trans: Transaction) {
        // if there were no observers, we don't care
        if let Some(on_settled) = self.trans_observers.remove(&trans) {
            on_settled(trans)
        }
    }

    pub fn add_block_listener<CallBack: 'static + FnMut(Block)>(
        &mut self,
        on_block: CallBack,
    ) -> &mut Self {
        self.block_observers.push(Box::new(on_block));
        self
    }

    pub fn notify_block_event(&mut self, block: Block) -> &mut Self {
        self.block_observers
            .iter_mut()
            .for_each(|notify| notify(block.clone()));

        block
            .transactions
            .into_iter()
            .for_each(|trans: Transaction| self.notify_transaction_settled(trans));
        self
    }

    #[warn(dead_code)]
    pub fn add_block(&mut self, block: Block) {
        self.chain.blocks.push(block);
        self.notify_block_event(self.chain.blocks.last().expect("block must exist").clone());
    }
}

pub fn build_path_for_block(block: &Block) -> String {
    let block_id = block.id();
    let hash_value = block_id.split(":").last().unwrap();
    use std::env;

    String::from(format!(
        "{}.json",
        env::temp_dir().join(hash_value).to_str().unwrap()
    ))
}

pub fn write_block(block: &Block) -> Result<()> {
    use std::fs::File;
    let path = build_path_for_block(&block);
    Ok(serde_json::to_writer(&File::create(path)?, &block)?)
}

pub fn write_keypair(path: &str, data: &[u8; 64]) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("cannot open file");

    file.write_all(data).expect("write failed");
}

pub fn read_keypair(path: &str) -> io::Result<[u8; 64]> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0u8; 64];
    let n = file.read(&mut buf)?;
    if n == 64 {
        Ok(buf)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "invalid length"))
    }
}

pub fn get_keyfile_name(key_filename: &str) -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(key_filename);
    let filepath = path.into_os_string().into_string().unwrap();
    filepath
}

pub fn create_ed25519_keypair(path: &str) -> libp2p::identity::ed25519::Keypair {
    let filename = get_keyfile_name(path);
    debug!("Get Keypair File Name: {:?}", filename);
    match read_keypair(&filename) {
        Ok(v) => {
            let data: &mut [u8] = &mut v.clone();
            debug!("Load Keypair from {:?}", filename);
            libp2p::identity::ed25519::Keypair::decode(data).unwrap()
        }
        Err(_) => {
            let id_keys = identity::ed25519::Keypair::generate();

            let data = id_keys.encode();
            debug!("Create Keypair");
            write_keypair(&filename, &data);
            id_keys
        }
    }
}

#[test]
fn generate_genesis() {
    println!("Start method");
    let keypair = create_ed25519_keypair("keypair");
    let local_id = Address::from(identity::PublicKey::Ed25519(keypair.public()));
    let transaction = Transaction::new(
        local_id, // need to load from local file
        json!({
            "type" : "AddAuthority",
            "key" : data_encoding::BASE64.encode(&keypair.public().encode())
        }),
        &BlockKeypair(keypair.clone()),
    );
    let block = Block::new(
        HashDigest::new("".as_bytes()),
        0,
        Vec::from([transaction]),
        &BlockKeypair(keypair.clone()),
    );
    println!("Hello, World!");
    println!("{}", block);
    // as JSON. We then need to hardcode this output as the genesis block
    debug!("{}", block);
}

#[cfg(test)]
mod tests {
    use crate::args::parser::{BlockchainNodeArgs, DEFAULT_BLOCK_KEYPAIR_FILENAME};
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;

    #[derive(Serialize, Clone, Eq, PartialEq, Debug, Deserialize)]
    struct Thing {
        name: String,
        age: usize,
    }

    #[test]
    fn test_build_blockchain() {
        let keypair: Keypair = Keypair::generate();
        let mut chain = Blockchain::new(&BlockKeypair(keypair.clone()));
        println!("Public key {:?}", keypair.public());
        let trans: Transaction = chain.submit_transaction("Hello First Transaction", |_| {});
        chain.add_block_listener(move |b: Block| {
            assert!(b.verify());
        });
        chain.save();
    }

    #[test]
    fn test_add_trans_listener() {
        let keypair = Keypair::generate();
        let mut bc = Blockchain::new(&BlockKeypair(keypair));

        let called = Rc::new(Cell::new(false));
        let data = Thing {
            name: String::from("Christian"),
            age: 10,
        };
        bc.submit_transaction(data.clone(), {
            let called = called.clone();
            let d = data.clone();
            move |t: Transaction| {
                let result: Thing = serde_json::from_value(t.payload()).unwrap();
                assert_eq!(d, result);
                called.set(true)
            }
        });
        bc.save();
        assert!(called.get());
    }

    #[test]
    fn test_add_block_listener() {
        let keypair = Keypair::generate();

        let mut chain = Blockchain::new(&BlockKeypair(keypair));
        let called = Rc::new(Cell::new(false));

        chain
            .add_block_listener({
                let called = called.clone();
                move |b: Block| {
                    let result: Thing =
                        serde_json::from_value(b.transactions.last().unwrap().payload()).unwrap();
                    assert_eq!(
                        Thing {
                            name: String::from("christian"),
                            age: 10
                        },
                        result
                    );
                    called.set(true);
                }
            })
            .submit_transaction(
                Thing {
                    name: String::from("christian"),
                    age: 10,
                },
                |_| {},
            );
        chain.save();

        assert!(called.get()); // called is still false
    }

    const TEST_KEYPAIR_FILENAME: &str = "./test_keypair";

    #[test]
    fn test_get_keyfile_name_succeeded() {
        let mut path = dirs::home_dir().unwrap();

        path.push(DEFAULT_BLOCK_KEYPAIR_FILENAME);
        let args = BlockchainNodeArgs::new();
        assert_eq!(
            path.into_os_string().into_string().unwrap(),
            get_keyfile_name(&args.key_filename)
        );
    }

    #[test]
    fn test_write_keypair_succeeded() {
        let file = String::from(TEST_KEYPAIR_FILENAME);
        let data = [0u8; 64];
        let result = std::panic::catch_unwind(|| write_keypair(&file, &data));
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_keypair_succeeded() {
        let file = String::from(TEST_KEYPAIR_FILENAME);
        let data = [0u8; 64];
        write_keypair(&file, &data);
        assert!(read_keypair(&file).is_ok());
    }

    #[test]
    fn test_create_keypair_succeeded() {
        let args = BlockchainNodeArgs::new();
        let result = std::panic::catch_unwind(|| create_ed25519_keypair(&args.key_filename));
        assert!(result.is_ok());
    }
}

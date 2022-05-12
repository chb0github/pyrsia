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

use std::io::{stdin, BufRead, BufReader};
use std::{
    error::Error,
};

// use pyrsia_blockchain_network::blockchain::Blockchain;
use pyrsia_blockchain_network::blockchain::{create_ed25519_keypair, Blockchain};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // If the key file exists, load the key pair. Otherwise, create a random keypair and save to the key file
    // let id_keys = create_ed25519_keypair();
    // If the key file exists, load the key pair. Otherwise, create a random keypair and save to the keypair file
    let keypair = create_ed25519_keypair("keypair");
    let mut bc = Blockchain::new(&keypair);

    BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap())
        .for_each(|l| {
            bc.submit_transaction(l.as_bytes().to_vec(), |t| {
                println!("transaction  accepted {}", t.signature().as_string());
            });
        });
<<<<<<< HEAD
    pretty_env_logger::init();

    let args = BlockchainNodeArgs::parse();

    let key_path = get_keyfile_name(args.clone());

    // If the key file exists, load the key pair. Otherwise, create a random keypair and save to the keypair file
    let id_keys = create_ed25519_keypair(key_path);
    let ed25519_pair = identity::Keypair::Ed25519(id_keys.clone());
    let _peer_id = PeerId::from(ed25519_pair.public());

    info!("Getting network up!");
    let n_members = 3;
    let my_node_ix = NodeIndex(args.peer_index);

    Ok(())
}

pub fn write_block(path: &str, block: Block) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("cannot open file");

    file.write_all(serde_json::to_string(&block).unwrap().as_bytes())
        .expect("write failed");
    file.write_all(b"\n").expect("write failed");
}

pub fn write_keypair(path: &String, data: &[u8; 64]) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o600)
        .open(path)
        .expect("cannot open file");

    file.write_all(data).expect("write failed");
}

pub fn read_keypair(path: &String) -> Result<[u8; 64], Box<dyn Error>> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0u8; 64];
    let n = file.read(&mut buf)?;
    if n == 64 {
        Ok(buf)
    } else {
        Err(Box::new(io::Error::from(io::ErrorKind::InvalidData)))
    }
}

pub fn get_keyfile_name(args: BlockchainNodeArgs) -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(args.key_filename);
    let filepath = path.into_os_string().into_string().unwrap();
    filepath
}

pub fn create_ed25519_keypair(filename: String) -> libp2p::identity::ed25519::Keypair {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pyrsia_blockchain_network::args::parser::DEFAULT_BLOCK_KEYPAIR_FILENAME;
    const TEST_KEYPAIR_FILENAME: &str = "./test_keypair";
    #[test]
    fn test_get_keyfile_name_succeeded() {
        let mut path = dirs::home_dir().unwrap();

        path.push(DEFAULT_BLOCK_KEYPAIR_FILENAME);
        let args = BlockchainNodeArgs {
            key_filename: DEFAULT_BLOCK_KEYPAIR_FILENAME.to_string(),
            peer_index: 0,
        };
        assert_eq!(
            path.into_os_string().into_string().unwrap(),
            get_keyfile_name(args)
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
        let args = BlockchainNodeArgs {
            key_filename: DEFAULT_BLOCK_KEYPAIR_FILENAME.to_string(),
            peer_index: 0,
        };
        let result = std::panic::catch_unwind(|| create_ed25519_keypair(args));
        assert!(result.is_ok());
    }
}

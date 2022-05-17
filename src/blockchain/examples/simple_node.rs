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
use libp2p::identity::ed25519::SecretKey;

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

    Ok(())
}

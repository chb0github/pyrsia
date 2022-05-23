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

use std::error::Error;
use std::io::{stdin, BufRead, BufReader};

use pyrsia_blockchain_network::blockchain::{create_ed25519_keypair, Blockchain};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
struct Thing {
    name: String,
    age: usize,
}

///
/// The main function's only job is to read from stdin and bulk up transactions
/// When you're ready to save them all to a block type 'save'. At this moment, files only
/// write to disk when the app exits - currently unknown why.
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let keypair = create_ed25519_keypair("keypair");
    let mut bc = Blockchain::new(&keypair);

    BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap())
        .for_each(|l| {
            match l.as_str() {
                "save" => {
                    bc.save()
                }
                _ => {
                    let thing = Thing {
                        name: l,
                        age: 10
                    };
                    bc.submit_transaction(thing, |t| {
                        println!("transaction  accepted {}", t.signature().as_string());
                    });
                }
            }
        });

    Ok(())
}

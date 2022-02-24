use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::block::{Block, Transaction};
use crate::blockchain::Blockchain;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct BlockchainError;

// But we require certain bounds to get things done...
impl Blockchain {
    // should we borrow or own this transaction?
    pub fn submit_transaction<CallBack: 'static + FnOnce(Transaction)>(
        &mut self,
        trans: Transaction,
        on_done: CallBack,
    ) -> &mut Self {
        self.trans_observers.insert(trans, Box::new(on_done));
        self
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
        self
    }
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Debug for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Blockchain {
            trans_observers: _,
            genesis_block,
            blocks,
            block_observers: _,
        } = self;

        f.debug_struct("Blockchain")
            .field("genesis_block", genesis_block)
            .field("blocks", blocks)
            .finish()
    }
}

mod test {
    use std::cell::Cell;
    use std::rc::Rc;

    use libp2p::identity;
    use rand::Rng;

    use crate::block::*;
    use crate::blockchain::generate_ed25519;
    use crate::blockchain::Blockchain;
    use crate::header::{hash, Header, PartialHeader};

    #[test]
    fn test_add_trans_listener() -> Result<(), String> {
        let keypair = generate_ed25519();
        let ed25519_keypair = match keypair {
            identity::Keypair::Ed25519(v) => v,
            identity::Keypair::Rsa(_) => todo!(),
            identity::Keypair::Secp256k1(_) => todo!(),
        };
        let local_id = hash(&get_publickey_from_keypair(&ed25519_keypair).encode());
        let mut chain = Blockchain::new(&ed25519_keypair);

        let transaction = Transaction::new(
            PartialTransaction::new(
                TransactionType::Create,
                local_id,
                "some transaction".as_bytes().to_vec(),
                rand::thread_rng().gen::<u128>(),
            ),
            &ed25519_keypair,
        );
        let called = Rc::new(Cell::new(false));
        chain
            .submit_transaction(transaction.clone(), {
                let called = called.clone();
                let transaction = transaction.clone();
                move |t: Transaction| {
                    assert_eq!(transaction, t);
                    called.set(true)
                }
            })
            .notify_transaction_settled(transaction);
        assert!(called.get());
        Ok(())
    }

    #[test]
    fn test_add_block_listener() -> Result<(), String> {
        let ed25519_keypair = match generate_ed25519() {
            identity::Keypair::Ed25519(v) => v,
            identity::Keypair::Rsa(_) => todo!(),
            identity::Keypair::Secp256k1(_) => todo!(),
        };
        let local_id = hash(&get_publickey_from_keypair(&ed25519_keypair).encode());

        let block_header = Header::new(PartialHeader::new(
            hash(b""),
            local_id,
            hash(b""),
            1,
            rand::thread_rng().gen::<u128>(),
        ));

        let block = Block::new(
            block_header,
            Vec::new(),
            &identity::ed25519::Keypair::generate(),
        );
        let mut chain = Blockchain::new(&ed25519_keypair);
        let called = Rc::new(Cell::new(false));

        chain
            .add_block_listener({
                let called = called.clone();
                let block = block.clone();
                move |b: Block| {
                    assert_eq!(block, b);
                    called.set(true);
                }
            })
            .notify_block_event(block);
        assert!(called.get()); // called is still false
        Ok(())
    }
}

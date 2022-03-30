- Feature Name: (fill me in with a unique ident, `my_awesome_feature`)
- Start Date: (fill me in with today's date, YYYY-MM-DD)
- RFC PR: [pyrsia/pyrsia#0000](https://github.com/pyrsia/pyrsia/pull/0000)
- Pyrsia Issue: [pyrsia/pyrsia#0000](https://github.com/pyrsia/pyrsia/issues/0000)

# Summary

What's in the Pyrsia blockchain and why

# Motivation

In order to make a robust blockchain, and give the community some guidance on our decisions, documenting each field
in the blockchain proves to be a worthwhile endeavour. In the process, we have discovered ways we can improve already on
what we have.

# Guide-level explanation

When designing our blockchain we didn't try to get clever. We tried to stand on the shoulders of giants and accepted most of the implementations
we found as inspired. Where possible, we tried to dig into reasons for any particular choice. But ultimately we decided that what we were seeing represented
state-of-the-art, and we should lean towards accepting it.

We looked at the source code for `ETH` and several minor OSS implementations.

# Prior art

When constructing this blockchain we consulted the structure of several famous blockchain implementations. Namely
`BTC` and `ETH`. Bitcoin was the simplest, but it has the worst growing pains and doesn't handle chain merging well.
Etherium manages chain merging by treating it not as a blockchain, but as a block tree - `Merkle tree`. In addition, all of the
new implementations of blockchain allow for a list of transactions in a single block. This keep the block count lower and thus
increases overall efficiency since it's the blocks that require consensus and take time.

TO that end, our blockchain contains:
1. Multiple transactions per block
2. Signed transaction hash for efficiently verifying transactions
3. Merkle root indicator so that we can use a merkle tree implementation

# Unresolved questions

None.

# Future possibilities

This structure is most definitely going to change as we learn and grow. There are already several outstanding tickets to do so

## A structural breakdown of our blockchain ##

Our blockchain consists

* series of blocks where
    * each block has
        * a Header
            * which contains it's parent hash. This is the part of the block chain that makes it a `chain`. Each block has a reference to it's parent going all the way back to the genesis. Basically an inductive proof.
            * The block committers address. On any block chain you need to identify the participant. The address isn't really any kind of routing destination or origin. It's the unique id of a participant and nothing more.
            * ~the transactions root - This is an optimization to verify all transactions in a block. We can simply add the transaction hash (256bit Keccak Hash of the root node of Transactions Merkle tree). Certain block chain implementations aren't blocks, they are actually merkle trees (this is done to handle chain merging conflicts)~
            * the timestamp of, in epoch time in seconds, of when the block was created
            * a header number is a monotonically increasing ordinal. Used to sort the blocks
            * a `nonce` for salting the payload
            * a Block header hash - a singular unique hash of all the data in the block header (except, of course, the signature)
        * a series of transactions
            * Each transaction has a type (`AddAuthority`, `RevokeAuthority`, `Generic`) - that help determine how the payload should be interpreted.
            * The submitter of the transaction, represented as a blockchain address
            * The timestamp of the transaction in epoch time
            * the actual payload as a series of bytes
            * a `nonce` used to salt the signature of the block
            * a transaction hash - digest of the payload
            * a transaction signature - signed version of the hash - optimization . Since the hash represents the the payload of the transaction, then signing the hash is a quick way to validate the whole transaction without recomputing it all.
        * a signature allowing for verification of the block

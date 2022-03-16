- Feature Name: (fill me in with a unique ident, `my_awesome_feature`)
- Start Date: (fill me in with today's date, YYYY-MM-DD)
- RFC PR: [pyrsia/pyrsia#0000](https://github.com/pyrsia/pyrsia/pull/0000)
- Pyrsia Issue: [pyrsia/pyrsia#0000](https://github.com/pyrsia/pyrsia/issues/0000)

# Summary

Exploration, reasoning and choice for a consensus engine to build on top of

# Motivation

One of Pyrsia's main design decisions is to use a blockchain for its underlying datastore. The data structure(s) of a
blockchain are plenty challenging but nothing mind bending. However, the real challenge to blockchain is consensus;
how do you get a bunch of strangers to trust each other and agree, when some of them might be corrupt and you could
have any number of faults in the system?

## Core Requirements
- Avoid any notion of monetary assets (e.g balance, account, tokens)
- Short TTF
- Apache 2 or compatible license (To avoid legal problems)
- Active community development (necessary for good support)

Given the complexity of consensus and the data structures used to manage a blockchain, it's advisable to find existing
code that can be leveraged; stand on the shoulders of giants, as the expression goes. However, this led us into a
contradiction with another requirement for Pyrsia; a legal and regulatory one: No currency. Each of these existing
libraries that implement blockchain consensus and data structures all assume there is a currency involved. After all,
this is the biggest reason people have gravitated toward blockchain: Bitcoin and Ethereum.

# Prior art

## Aleph

There was one library in particular that stood out for focusing on consensus and nothing else: [Aleph](https://github.com/aleph-zero-foundation/AlephBFT).
Also, the implementation was simple as only 3 [basic traits needed to be implemented](https://github.com/aleph-zero-foundation/AlephBFT#using-the-crate)
1. Sign/verify
2. I/O (distinct from network IO)
3. Network

A more detailed write-up is available [here](https://docs.google.com/presentation/d/1hJIa9Ht-s99CuHGou6Ra9Y7Lkk2OIV9OjxM4wVDFcao/edit#slide=id.g1156090f3c7_1_125)

In addition, because `verify` and `sign` as so cleanly isolated, we are able to use proof of authority. Which is simple and gives enterprise IT admins warm fuzzy feelings.
Finally, it's block uses very well tread data structures, that have been proven repeatedly.

## Substrate
Another such tech [substrate](https://substrate.io/) did much more, and had a compatible license, but had the
assumption of a currency. We had considered setting transaction costs and gas to 0 (thus neutralising block currency), but even this was deemed too much.

## Tendermint
The last tech truly considered was [tendermint](https://tendermint.com/). However, their examples wouldn't compile and
when reaching out to community, after taking extreme measures to get it working, the response was ["We don't have the
bandwidth to maintain docs in as many languages set of docs so we're planning on removing them"](https://github.com/tendermint/tendermint/issues/7743#issuecomment-1028025629).
In addition, a core part of their platform is _not_ license compatible.

# Unresolved questions

None.

# Future possibilities

This structure is most definitely going to change as we learn and grow. There are already several outstanding tickets to do so

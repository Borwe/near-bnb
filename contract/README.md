flats Smart Contracts
==================
A set of two smart contracts, one a factory that generates the other.

flat_factory
=============
Contract used to create flat_contract contracts, which in themselves are individual rooms or houses that represent a single bed and breakfast unit which includes it's own price, location, and availability dates which a user then interacts with. Only the Admin or owner of the flat/housing unit, interacts with this contract to produce a unit.

flat_contract
==============
Contract representing a single housing unit, which ordinary users can check for availability and book specific days. All payment records are stored and easily retrievable.


Exploring The Code
==================

1. The flats_factory contract is located in ./flats_factory.
2. The flats_contract contract is located in ./flats_contract.
2. Tests: You can run smart contract tests with the `./test` script. This runs
   standard Rust tests using [cargo] with a `--nocapture` flag so that you
   can see any debug info you print to the console.


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

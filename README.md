

# NEAR-BNB

An implementation for managing housing similar to how BNB works.

### How it works:

- Each owner of the house generates a contract for their own house.
- Each house, get's a contract.
- A user/renter can then book to rent the house at a specific date
- The user/renter should verify ownership at the place before entering.
- Currency used is NEAR.

### Requirements:

- Angular cli, for running tests
- node >=12 with npm.
- Rust and cargo, for compiling the contracts
- Near cli (install via `npm install --global near-cli`)

### Steps:

1. ```bash
   near login
   ```
   Login with the testnet account used for deploying and doing tests


2. ```bash
   npm install
   ```
   Installs the dependencies required to run tests and following steps


3. ```bash
   npm run setup
   ```

   Sets up the contract name from env file in **./neardev/dev-account.env** which should look like this:

   ```bash
   MAIN_CONTRACT_ADDRESS = "hse.borwe2.testnet";
   ```

   This will be the contract address to be used for deploying the contract after building, make sure the file exists, or it would default to `hse.borwe2.testnet` also make sure the MAIN_CONTRACT_ADDRESS, contains an actual existing address in testnet, otherwise it would fail. when it comes to deploying the contract.
   
   Make sure that the sub address matches your account that you used in previous command of **`near login`**, eg: if account was **borwe2.testnet**, then the main account address will be hse.**borwe2.testnet**

   
4. ```bash
   npm run deploy:contract
   ```

   This builds, the contracts, and deploys the main one, also requires the **./neardev/dev-account.env** to exist, from previous step.

   This command will prompt user for account to be used as owner of the main contract, this will be admin account id for that contract. **Pass in the account you used in near login for it to work**.

5. ```bash
   npm run test_service
   ```

   This will run the tests, and simulate what a user might perform on your contracts.


Exploring The Code
==================

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The frontend code lives in the `/src` folder. `/src/main.ts` is a great
   place to start exploring.
   can learn how the frontend connects to the NEAR blockchain.
3. Tests: there are different kinds of tests for the frontend and the smart
   contract. See `contract/README` for info about how it's tested. The frontend
   code gets tested with [karma + jasmine]. You can run both of these at once with `npm
   run test`.


Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


[Angular]: https://angularjs.org/
[create-near-app]: https://github.com/near/create-near-app
[Node.js]: https://nodejs.org/en/download/package-manager/
[karma + jasmine]: https://angular.io/guide/testing
[NEAR accounts]: https://docs.near.org/docs/concepts/account
[NEAR Wallet]: https://wallet.testnet.near.org/
[near-cli]: https://github.com/near/near-cli
[gh-pages]: https://github.com/tschaub/gh-pages

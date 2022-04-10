

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
- Near cli

### Steps:

1. ```bash
   npm run setup
   ```

   Setup the contract name from env file in **./neardev/dev-account.env** which should look like this:

   ```bash
   MAIN_CONTRACT_ADDRESS = "hse.borwe.testnet";
   ```

   This will be the contract address to be used for deploying the contract after building, make sure the file exists, or it would default to `hse.borwe.testnet` also make sure the MAIN_CONTRACT_ADDRESS, contains an actual existing address in testnet, otherwise it would fail. when it comes to deploying the contract.

2. ```bash
   npm run deploy:contract
   ```

   This builds, the contracts, and deploys the main one

3. ```bash
   npm run test_service
   ```

   This will run the tests, and simulate what a user might perform on your contracts.



Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `npm install`
3. Run the local development server: `npm run dev` (see `package.json` for a
   full list of `scripts` you can run with `npm`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.


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


Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `npm run dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `npm install`, but for best ergonomics you may want to install it globally:

    npm install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `flats.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `flats.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account flats.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Step 2: set contract name in code
---------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'flats.YOUR-NAME.testnet'


Step 3: deploy!
---------------

One command:

    npm run deploy

As you can see in `package.json`, this does two things:

1. builds & deploys smart contract to NEAR TestNet
2. builds & deploys frontend code to GitHub using [gh-pages]. This will only work if the project already has a repository set up on GitHub. Feel free to modify the `deploy` script in `package.json` to deploy elsewhere.


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

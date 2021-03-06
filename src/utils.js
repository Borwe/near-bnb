import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'
import getConfig from './config'

const nearConfig = getConfig('development')

console.log(nearConfig)

// Initialize contract & set global variables
export async function initContract(account=undefined) {
  // Initialize connection to the NEAR testnet
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near)

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId()

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(
    account!==undefined?account:window.walletConnection.account(),
    nearConfig.contractName, {
    changeMethods: ['create_house'],
    viewMethods: ['check_house_name_available',
     'get_all_houses','get_owner'],
  })
}

export async function setupHouseContract(
  house_contract_name, account=undefined){
  window.houseContract = await new Contract(
    account!==undefined?account:window.walletConnection.account(),
    house_contract_name, {
    changeMethods: ['book_house','verify'],
    viewMethods: ['check_date_available',
      'get_house_info', 'get_owner'],
  })
}

export function logout() {
  window.walletConnection.signOut()
  // reload page
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName)
}

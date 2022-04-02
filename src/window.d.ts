import { Contract, WalletConnection } from 'near-api-js'
import { Flat } from './app/models/Models';

interface MyContract extends Contract {
  create_flat(value: Flat, gas: string, pay: string): string 
  map_flat_contract_to_user_id(value: { 
    flat_owner: string,
    flat_account: string
  }): void

  check_flat_name_available(value: { flat_name: string }): boolean
  get_owner(): string
  get_all_flats(): Array<String>
}

declare global {
  interface Window {
    walletConnection: WalletConnection
    accountId: string
    contract: MyContract
    nearInitPromise: Promise<void>
  }
}

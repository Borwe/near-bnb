import { Contract, WalletConnection } from 'near-api-js'
import { House, HouseName } from './app/models/Models';

interface MyContract extends Contract {
  create_house(value: House, gas: string, pay: string): string 
  check_house_name_available(value: HouseName ): boolean
  get_owner(): string
  get_all_houses(): Array<String>
}

declare global {
  interface Window {
    walletConnection: WalletConnection
    accountId: string
    contract: MyContract
    nearInitPromise: Promise<void>
  }
}

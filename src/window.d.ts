import { Contract, WalletConnection } from 'near-api-js'
import { Date , House, HouseName, HouseInfo } from './app/models/Models';

interface MainContract extends Contract {
  create_house(value: House, gas: string, pay: string): string 
  check_house_name_available(value: HouseName ): boolean
  get_owner(): string
  get_all_houses(): Array<String>
}

interface HouseContract extends Contract {
  book_house(date: Date, gas: string, pay: string): boolean;
  check_date_available(date: Date): boolean;
  get_owner(): boolean;
  verify(date: Date, gas: string, pay: string): boolean;
  get_house_info(): HouseInfo;
}

declare global {
  interface Window {
    walletConnection: WalletConnection
    accountId: string
    contract: MainContract
    houseContract: HouseContract
    nearInitPromise: Promise<void>
  }
}

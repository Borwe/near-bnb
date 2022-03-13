import { Contract, WalletConnection } from 'near-api-js'

interface MyContract extends Contract {
  set_greeting(value: { message: string }): void
  get_greeting(value: { account_id: string }): string | null
}

declare global {
  interface Window {
    walletConnection: WalletConnection
    accountId: string
    contract: MyContract
    nearInitPromise: Promise<void>
  }
}

import { Account } from 'near-api-js'

export declare function initContract(
  account: Account | undefined): Promise<void>

export function setupHouseContract(
  house_contract_name: string,
  account: Account | undefined): Promise<void>
export declare function login(): void
export declare function logout(): void

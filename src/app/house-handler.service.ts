import { Inject, Injectable } from '@angular/core';
import { WINDOW } from './services/window.service';
import { login, logout, setupHouseContract } from '../utils';
import { House, HouseInfo, HouseName, Date } from './models/Models';
import { utils, Account } from 'near-api-js'
import getConfig  from '../config'

@Injectable({
  providedIn: 'root'
})
export class HouseHandlerService {

  constructor(@Inject(WINDOW) private window: Window) { }

  public isSignedIn(){
    return this.window.walletConnection.isSignedIn();
  }

  public async getOwnerOfContract(): Promise<string>{
    return window.contract.get_owner();
  }

  public async setHouseContract(account: Account | undefined
    ,house: House, env:string){
    let contract_name = getConfig(env).contractName;
    contract_name = house.name+"."+contract_name;
    await setupHouseContract(contract_name,account);
  }

  public async checkHouseAvailable(date: Date): Promise<boolean>{
    return this.window.houseContract.check_date_available(date);
  }

  public async getHouseInfo(account: Account | undefined
    ,house: House, env:string): Promise<HouseInfo>{
    await this.setHouseContract(account, house, env);
    return this.window.houseContract.get_house_info();
  }

  public async verifyUserBooked(date: Date): Promise<boolean>{
    return this.window.houseContract.verify(date,
      "300000000000000", "0");
  }

  public async bookHouse(date: Date, amount: string): Promise<boolean>{
    return this.window.houseContract.book_house(date,
      "300000000000000", utils.format.parseNearAmount(amount));
  }

  public async createHouse(house: House): Promise<string>{
    return window.contract.create_house(
      house ,"300000000000000",utils.format.parseNearAmount("10"));
  }

  public async checkIfNameAvailable(house_name: HouseName): Promise<boolean>{
    return window.contract.check_house_name_available(house_name);
  }

  public async getAviailableHouses(): Promise<Array<String>>{
    return window.contract.get_all_houses();
  }

  public signInWallet(){
    login();
  }

  public signOutWallet(){
    logout();
  }
}

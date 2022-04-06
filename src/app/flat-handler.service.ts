import { Inject, Injectable } from '@angular/core';
import { WINDOW } from './services/window.service';
import { login, logout } from '../utils';
import { House, HouseName } from './models/Models';
import { utils } from 'near-api-js'

@Injectable({
  providedIn: 'root'
})
export class FlatHandlerService {

  constructor(@Inject(WINDOW) private window: Window) { }

  public isSignedIn(){
    return this.window.walletConnection.isSignedIn();
  }

  public async getOwnerOfContract(): Promise<string>{
    return window.contract.get_owner();
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

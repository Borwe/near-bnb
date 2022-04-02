import { Inject, Injectable } from '@angular/core';
import { WINDOW } from './services/window.service';
import { login, logout } from '../utils';
import { Flat } from './models/Models';
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

  public async createFlat(flat: Flat): Promise<string>{
    return window.contract.create_flat(
      flat ,"300000000000000",utils.format.parseNearAmount("10"));
  }

  public async checkIfNameAvailable(name: string): Promise<boolean>{
    return window.contract.check_flat_name_available({
      flat_name: name
    });
  }

  public async getAviailableFlats(): Promise<Array<String>>{
    return window.contract.get_all_flats();
  }

  public signInWallet(){
    login();
  }

  public signOutWallet(){
    logout();
  }
}

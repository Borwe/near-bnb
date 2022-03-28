import { Inject, Injectable } from '@angular/core';
import { WINDOW } from './services/window.service';
import { login, logout } from '../utils';

@Injectable({
  providedIn: 'root'
})
export class FlatHandlerService {

  constructor(@Inject(WINDOW) private window: Window) { }

  public isSignedIn(){
    return this.window.walletConnection.isSignedIn();
  }

  public signInWallet(){
    login();
  }

  public signOutWallet(){
    logout();
  }
}

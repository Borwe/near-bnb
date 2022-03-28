import { TestBed, waitForAsync } from '@angular/core/testing';
import { BrowserModule } from '@angular/platform-browser'

import { FlatHandlerService } from './flat-handler.service';
import {initContract} from '../utils'


import { setUpTestConnection, generateUniqueString, createAccount } from 'near-api-js/test/test-utils';

import {ACCOUNT, PRIVATE_KEY} from '../../neardev/test_obj';

import { WINDOW, WINDOW_PROVIDERS } from './services/window.service'
import { KeyPairEd25519 } from 'near-api-js/lib/utils';
import { keyStores, connect, Account} from 'near-api-js';

describe('FlatHandlerService', () => {
  let service: FlatHandlerService;
  let spyWindow: Window;
  let account: Account;

  beforeAll(waitForAsync(async (): Promise<void> => {

    TestBed.configureTestingModule({
      imports: [BrowserModule, ],
      providers: [
        WINDOW_PROVIDERS,FlatHandlerService
      ]
    }).compileComponents();
    await initContract();
    spyWindow = TestBed.inject(WINDOW) as Window;

    let browserkeys = new keyStores.BrowserLocalStorageKeyStore();
    await browserkeys.setKey( spyWindow.walletConnection._networkId,
      ACCOUNT, KeyPairEd25519.fromString(PRIVATE_KEY));

    let config = {
      networkId: "testnet",
      keyStore: browserkeys,
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
      explorerUrl: "https://explorer.testnet.near.org"
    }

    let near = await connect(config);
    account = await near.account(ACCOUNT);

    service = TestBed.inject(FlatHandlerService);
  }))



  it('Test if test variables read',()=>{
    expect(ACCOUNT.length>0).toBeTrue();
    expect(PRIVATE_KEY.length>0).toBeTrue();
  })

  it('Test if account is valid', async ()=>{
    expect(await account.state()).toBeTruthy();
    expect(account.accountId === ACCOUNT ).toBeTrue();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

});

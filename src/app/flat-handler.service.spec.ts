import { TestBed, waitForAsync } from '@angular/core/testing';
import { BrowserModule } from '@angular/platform-browser'

import { FlatHandlerService } from './flat-handler.service';
import {initContract} from '../utils'


import { setUpTestConnection, generateUniqueString, createAccount } from 'near-api-js/test/test-utils';

import {ACCOUNT, PRIVATE_KEY} from '../../neardev/test_obj';

import { WINDOW, WINDOW_PROVIDERS } from './services/window.service'
import { KeyPairEd25519 } from 'near-api-js/lib/utils';
import { keyStores, connect, Account, utils} from 'near-api-js';
import { Flat } from './models/Models';

describe('FlatHandlerService', () => {
  let service: FlatHandlerService;
  let spyWindow: Window;
  let account: Account;

  beforeAll(waitForAsync(async (): Promise<void> => {

    jasmine.DEFAULT_TIMEOUT_INTERVAL = 100000000000;

    TestBed.configureTestingModule({
      imports: [BrowserModule, ],
      providers: [
        WINDOW_PROVIDERS,FlatHandlerService
      ]
    }).compileComponents();


    let browserkeys = new keyStores.BrowserLocalStorageKeyStore();
    await browserkeys.setKey( "testnet",
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

    await initContract(account);
    spyWindow = TestBed.inject(WINDOW) as Window;


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

  it('Try get owner of the factory contract', async()=>{
    expect(await service.getOwnerOfContract()==="borwe.testnet").toBeTrue();
  })

  it("Try check if flat contract exists",async()=>{
    let random_name_might_exist = "vescon_ke254";
    if(await service.checkIfNameAvailable(random_name_might_exist) === true){
      //create a flat
      let flat = new Flat();
      flat.name = random_name_might_exist;
      flat.rooms = "300";
      flat.price =  1000;
      flat.location = "1.00,1.00";
      flat.features = "Fuck yeah, Cool";
      flat.image = "https://google.com";
      console.log("FLAT: ",flat);
      await service.createFlat(flat);
      //and now the check should fail
      expect(await service
        .checkIfNameAvailable(random_name_might_exist) == false)
        .toBeTrue();
    }
  })

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

});

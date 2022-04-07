import { TestBed, waitForAsync } from '@angular/core/testing';
import { BrowserModule } from '@angular/platform-browser'

import { HouseHandlerService } from './house-handler.service';
import {initContract} from '../utils'


import { setUpTestConnection, generateUniqueString, createAccount } from 'near-api-js/test/test-utils';

import {ACCOUNT, PRIVATE_KEY} from '../../neardev/test_obj';

import { WINDOW, WINDOW_PROVIDERS } from './services/window.service'
import { KeyPairEd25519 } from 'near-api-js/lib/utils';
import { keyStores, connect, Account, utils} from 'near-api-js';
import { House, HouseName } from './models/Models';

describe('FlatHandlerService', () => {
  let service: HouseHandlerService;
  let spyWindow: Window;
  let account: Account;

  beforeAll(waitForAsync(async (): Promise<void> => {

    jasmine.DEFAULT_TIMEOUT_INTERVAL = 100000000000;

    TestBed.configureTestingModule({
      imports: [BrowserModule, ],
      providers: [
        WINDOW_PROVIDERS,HouseHandlerService
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


    service = TestBed.inject(HouseHandlerService);
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

  it("Try check if flat contract exists, and try getting list of contracts",async()=>{
    let version = Math.floor(Math.random()*1000);
    let houseName = new HouseName();
    houseName.house_name = "vescon_build_"+version;
    while(await service.checkIfNameAvailable(houseName) === false){
      version = Math.floor(Math.random()*1000);
      houseName.house_name = "vescon_build_"+version;

    }
    //create a flat
    let house = new House();
    house.name = houseName.house_name;
    house.price =  utils.format.parseNearAmount("10");
    house.location = "1.00,1.00";
    house.features = "Fuck yeah, Cool";
    house.image = "https://google.com";
    await service.createHouse(house);
    //and now the check should fail
    expect(await service
      .checkIfNameAvailable(houseName) == false)
      .toBeTrue();
    
    let flats: Array<String> = await service.getAviailableHouses();
    expect(flats.length>0).toBeTrue();
    expect(flats.
      find((contract)=> contract.toLowerCase() === house.name.toLowerCase())!== undefined)
      .toBeTrue();
  })


});

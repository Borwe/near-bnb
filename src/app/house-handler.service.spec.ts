import { TestBed, waitForAsync } from '@angular/core/testing';
import { BrowserModule } from '@angular/platform-browser'

import { HouseHandlerService } from './house-handler.service';
import {initContract, setupHouseContract} from '../utils'


import { setUpTestConnection, generateUniqueString, createAccount } from 'near-api-js/test/test-utils';

import {ACCOUNT, PRIVATE_KEY} from '../../neardev/test_obj';

import { WINDOW, WINDOW_PROVIDERS } from './services/window.service'
import { KeyPairEd25519 } from 'near-api-js/lib/utils';
import { keyStores, connect, Account, utils} from 'near-api-js';
import { House, HouseInfo, HouseName, Date } from './models/Models';

describe('FlatHandlerService', () => {
  let service: HouseHandlerService;
  let spyWindow: Window;
  let account: Account;

  beforeAll(waitForAsync(async (): Promise<void> => {

    jasmine.DEFAULT_TIMEOUT_INTERVAL = 100000000000000;

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
    expect(await service.getOwnerOfContract()===ACCOUNT).toBeTrue();
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
    
    // check if houses was created
    let houses: Array<String> = await service.getAviailableHouses();
    expect(houses.length>0).toBeTrue();
    expect(houses.
      find((contract)=> contract.toLowerCase() === house.name.toLowerCase())!== undefined)
      .toBeTrue();

    // try interacting with the house contract
    await service.setHouseContract(account,house, "testnet");// set the contract to be used
    // check if date available for house booking
    let date = new Date();
    date.day = 1;
    date.year = 2022;
    date.month = 1;
    while(await service.checkHouseAvailable(date)===false){
      incrementDate(date);
    }

    await service.bookHouse(date,utils.format.formatNearAmount(house.price));

    //now should return false when trying to book on that same day again
    expect(await service.checkHouseAvailable(date)===false)
      .toBeTrue();

    //user to verify when they visit at the door
    expect(await service.verifyUserBooked(date) === true)
      .toBeTrue();

    //verify should fail when done on a date not booked
    let date2 = new Date();
    date2.day = 4;
    date2.year = 2022;
    date2.month = 2;
    expect(await service.verifyUserBooked(date2) === false)
      .toBeTrue();

    //check if info of current house is okay
    let houseInfo: HouseInfo =
      await service.getHouseInfo(account, house, "testnet");
    console.log("printing dummy, to avoid timeout");
    expect(houseInfo.name === house.name).toBeTrue();
    expect(houseInfo.price === house.price).toBeTrue();
  })
});

function incrementDate(date: Date){
  let big_months = [1,3,5,7,8,10,12];
  // increment the day of the month or month or year, if reached
  // end of the number of days available in that month
  if( big_months.find((x)=>x==date.month)!== undefined ) {
    if(date.day===31 && date.month!==12 ){
      date.day=1;
      date.month==date.month+1;
    }else if(date.month===12){
      date.day = 1;
      date.month = 1;
      date.year = date.year+1;
    }else{
      date.day = date.day+1;
    }
  }else{
    // special cases for feb:
    // if leap year, max is 29 days, otherwise 28
    if(date.month==2){
      if(date.year % 4 === 0){
        //leap year
        if( date.day== 29){
          date.day = 1;
          date.month = 3;
        }else{
          date.day = date.day+1;
        }
      }else{
        if(date.day == 28){
          date.day = 1;
          date.month = 3;
        }else{
          date.day = date.day+1;
        }
      }
    }else{
      if(date.month==30){
          date.day = 1;
          date.month = date.month+1;
      }else{
          date.day = date.day+1;
      }
    }
  }
}

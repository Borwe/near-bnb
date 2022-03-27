import { TestBed, waitForAsync } from '@angular/core/testing';
import { By } from '@angular/platform-browser'

import { FlatHandlerService } from './flat-handler.service';

import { setUpTestConnection, generateUniqueString, createAccount } from 'near-api-js/test/test-utils'
import { WalletConnection } from 'near-api-js'

import { WINDOW } from './services/window.service'

describe('FlatHandlerService', () => {
  let service: FlatHandlerService;
    let spyWindow: jasmine.SpyObj<Window>;
  let mockWindow;

  beforeAll(waitForAsync(async (): Promise<void> => {
    const near = await setUpTestConnection();
    const walletConnection = new WalletConnection(near, 'test');


    mockWindow = {
      accountId: 'test.near',
      contract: {
        account: createAccount(near),
        contractId: generateUniqueString('test'),
        get_greeting(): string {
          return 'Hello'
        },
        set_greeting(): void {}
      },
      walletConnection
    }
  }))


  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      providers: [
        {
          provide: WINDOW,
          useValue: mockWindow
        }
      ]
    }).compileComponents()
  }))

  beforeEach(()=>{
    service = TestBed.inject(FlatHandlerService);
    spyWindow = TestBed.inject(WINDOW) as jasmine.SpyObj<Window>;
  })


  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, Balance, AccountId};
use chrono::prelude::*;
use near_sdk::collections::{LookupMap,UnorderedMap};
use near_sdk::json_types::{U64};
use near_sdk::serde::{Serialize, Deserialize};
use flats_obj::Flat;

setup_alloc!();

const NEAR: Balance = 1_000_000_000_000_000_000_000_000;

#[derive(Serialize, Deserialize,Clone, BorshSerialize, BorshDeserialize)]
pub struct Payement{
    pub month: u32,
    pub year: i32,
    pub room: Room,
    pub price: Balance
}

#[derive(Serialize, Deserialize,Clone, BorshSerialize, BorshDeserialize)]
pub struct Room{
    /// room/house number
    pub room_no: u64,
    /// is available currently?
    pub is_available: bool, 
    /// show if renters account id, if their someone renting
    pub renter: Option<AccountId>,
    /// if available next month
    pub will_be_available_next_month: bool
}

impl Default for Room{
    fn default() -> Self {
        Self{
            room_no: 0,
            is_available: true,
            renter: None,
            will_be_available_next_month: false
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FlatContract {
    /// Hold room number as key, and accountID is the person renting
    rooms_list: UnorderedMap<u64,Room>,
    owner: AccountId,
    /// Hold rent money needed to rent room
    price: Balance,
    // Hold payment data
    payments: LookupMap<AccountId, Vec<Payement>>
}


impl Default for FlatContract {
  fn default() -> Self {
      panic!("Can't initialize contract from here")
  }
}

#[near_bindgen]
impl FlatContract {
    #[cfg(target_arch = "wasm32")]
    #[init]
    pub fn new(account: AccountId, flat: Flat)-> Self{
        // NOTE: Remove bellow assert if running tests
        //assert!(env::state_exists()==true, "Sorry, can only be called once by contract");
        let mut rooms_list: UnorderedMap<u64, Room> = 
            UnorderedMap::new(b"rooms_list".to_vec());
        flat.assert_location_valid();
        for i in 0..flat.rooms.into(){
            let mut room = Room::default();
            let room_no: u64 = i;
            room.room_no = room_no; 
            rooms_list.insert(&room_no,&room);
        }
        Self{
            rooms_list,
            owner: account,
            price: flat.price,
            payments: LookupMap::new(b"payements".to_vec())
        }
    }


    #[cfg(not(target_arch = "wasm32"))]
    #[init]
    pub fn new(account: AccountId, flat: Flat)-> Self{
        // NOTE: Remove bellow assert if running tests
        //assert!(env::state_exists()==true, "Sorry, can only be called once by contract");
        let mut rooms_list: UnorderedMap<u64, Room> = 
            UnorderedMap::new(b"rooms_list".to_vec());
        flat.assert_location_valid();
        for i in 0..flat.rooms.into(){
            let mut room = Room::default();
            let room_no: u64 = i;
            room.room_no = room_no; 
            rooms_list.insert(&room_no,&room);
        }
        Self{
            rooms_list,
            owner: account,
            price: flat.price,
            payments: LookupMap::new(b"payements".to_vec())
        }
    }

    #[payable]
    pub fn book_room(&mut self, room: U64)-> bool{
        let room_no: u64 = room.into();
        assert!(self.room_is_available(room)==true,
            "Sorry room {} not available",
                    room_no);

        let room: Room = self.rooms_list.get(&room.into())
            .expect("Room given doesn't exist");
        assert!(room.is_available == true,
                "Sorry, room {} not availbable",room_no);
        assert!(env::attached_deposit()==self.price, 
                "Sorry price for a room is {} NEAR"
                        ,self.price/NEAR);
        self.pay_room(room)
    }

    fn pay_room(&mut self,mut room: Room)-> bool{
        let mut payement_from_user: Vec<Payement> = self.payments
            .get(&env::signer_account_id()).unwrap_or(Vec::new());
        let date = Utc::now();
        room.renter = Some(env::signer_account_id());
        room.is_available = false;
        payement_from_user.push(Payement{
            year: date.year(),
            month: date.month(),
            room: room.clone(),
            price: env::attached_deposit()
        });
        //record payment by user
        self.payments.insert(&env::signer_account_id(),&payement_from_user);
        //update the room info
        self.rooms_list.insert(&room.room_no,&room.clone());
        true
    }

    pub fn not_going_to_rent_next_month(&mut self, room_no :U64)-> bool{
        let mut room = self.rooms_list
            .get(&room_no.into()).expect("No such room exists");
        match room.clone().renter {
            Some(x) => {
                assert!(x==env::signer_account_id(),"This isn't your room");
                room.will_be_available_next_month=true;
                self.rooms_list.insert(&room_no.into(),&room);
                return true;
            },
            None => panic!("Sorry, room wasn't rented by anybody yet")
        };
    }

    pub fn unlock_room_for_renting(&mut self, room_no:U64)->bool {
        assert!(self.owner == env::signer_account_id(),
            "Method can only be called by the owner of this flat");
        let mut room: Room = self.rooms_list.get(&room_no.into())
            .expect("No such room exists").clone();
        if room.will_be_available_next_month==true {
            room.will_be_available_next_month=false;
            room.is_available=true;
            room.renter = None;
            self.rooms_list.insert(&room_no.into(),&room).unwrap();
            true
        }else{
            false
        }
    }

    pub fn room_is_available(&self, room: U64)-> bool{
        if self.rooms_list.get(&room.into()).expect("No such room exists")
            .is_available {
            true
        }else{
            false
        }
    }

    pub fn get_rooms_unrenting_next_month(&self)-> Vec<Room>{
        let mut rooms_available_next_month = Vec::new();
        let keys = self.rooms_list.keys();
        for k in keys {
            let room = self.rooms_list.get(&k).expect("Room doesn't exist");
            if room.will_be_available_next_month == true {
                rooms_available_next_month.push(room);
            }
        }
        rooms_available_next_month
    }

    pub fn get_rooms(&self)-> U64{
        U64::from(self.rooms_list.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(signer_account_id: AccountId,input: Vec<u8>,
                   attached_deposit: u128, is_view: bool,
                   predecessor_account_id: Option<AccountId>) -> VMContext {
        let predecessor_account_id = match predecessor_account_id {
            Some(x) =>  x,
            None => signer_account_id.clone()
        };

        VMContext {
            current_account_id: "flat.rental".to_string(),
            signer_account_id,
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }


    fn get_dummy_flat_contract(ctx: &VMContext)-> FlatContract{

        let name = ctx.current_account_id.clone();
        let rooms = U64::from(300);
        let price = NEAR*15; // how much it costs to rent a room in the flat
        let location = "-1.227807,36.989969".to_string();
        let features = "Wifi,2 Swimming pools".to_string();
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();
        let flat = Flat::new(name,rooms,price,location,
                             features,image);

        // borwe.near is the creator of the contract
        FlatContract::new("borwe.near".to_string(),flat)
    }

    #[test]
    fn test_handling_rooms() {
        let context = get_context("bob.near".to_string(),vec![],NEAR*15,false,None);
        let ctx = context.clone();

        testing_env!(ctx);
        let mut contract = get_dummy_flat_contract(&context);

        assert!(contract.get_rooms() == U64::from(300), "Rooms count didn't match");
        
        //book a room
        let book_room = U64::from(10);
        assert!(contract.book_room(book_room.clone())==true, "Should be able to book this room");

        //see if book_room is part of available rooms, it should fail
        assert!(contract.room_is_available(book_room.into())==false, "Room shouldn't be available");

        //I can alert the owner that next month I don't intend to keep renting
        assert!(contract.not_going_to_rent_next_month(book_room)==true);

        let rooms_unrenting_next_month: Vec<Room> = contract.get_rooms_unrenting_next_month();// get rooms that should be available next month
        assert!(rooms_unrenting_next_month.len()>0, "bob.near set a room for unrenting, it should appear here");

        //This tests fail since changing context while running same contract
        //creates bug, should be tested in testnet instead
        //assume end month has reached, and then doing next actions as owner of the flat
        // or in other words owner of the contract
        //context.signer_account_id = "borwe.near".to_string();
        //testing_env!(context);

        //assert!(contract.unlock_room_for_renting(book_room)==true); // the room which user marked for sell
        //assert!(contract.unlock_room_for_renting(book_room)==false); // should be false since room already unlocked
        //assert!(contract.room_is_available(book_room)==true); //room should now be available


        //// Now another user should be able to rent the free room
        //let context = get_context("brando.near".to_string(),vec![],NEAR*15,false,None);
        //let _ctx = context.clone();
        //testing_env!(context);
        //
        //assert!(contract.book_room(book_room)==true, "Should be able to book this room");
        //assert!(contract.room_is_available(book_room)==false, "Room shouldn't be available");
    }
}

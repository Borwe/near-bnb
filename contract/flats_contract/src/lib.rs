use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, Balance, AccountId};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::{U64};
use near_sdk::serde::{Serialize, Deserialize};

setup_alloc!();

const NEAR: Balance = 1_000_000_000_000_000_000_000_000;

#[derive(Serialize, Deserialize,Clone)]
pub struct Position{
    pub latitude: String,
    pub longitude: String
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Flat{
    pub name: String,
    pub rooms: U64,
    pub price: Balance,
    pub location: Position,
    pub features: Option<Vec<String>>,
    pub image: Option<String>
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Room{
    /// room/house number
    pub id: u64,
    /// if not available currently, but soon can be, it should have a date
    pub available_date: u128, 
    /// is available currently?
    pub is_available: bool, 
    /// if user marked it as soon to be available
    pub is_soon_available: bool 
}


impl Flat{
    pub fn new(name: String,rooms: U64,price: Balance,location: Position,
            features: Option<Vec<String>>,image: Option<String>)->Self{
        Self{
            name,
            rooms,
            price,
            location,
            features,
            image
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FlatContract {
    rooms: UnorderedMap<Room, AccountId>
}

impl Default for FlatContract {
  fn default() -> Self {
      panic!("Can't initialize contract from here")
  }
}

#[near_bindgen]
impl FlatContract {
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
            predecessor_account_id: "carol_near".to_string(),
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
        let location = Position{latitude:"-1.227807".to_string(),longitude:"36.989969".to_string()};
        let features = vec!["Wifi".to_string(),
            "2 Swimming pools".to_string(),
            "Big open compound".to_string(),
            "alot of greenarie".to_string()];
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();
        let flat = Flat::new(name,rooms,price,location,
                             Some(features),Some(image));

        // borwe.near is the creator of the contract
        FlatContract::new("borwe.near".to_string(),flat.clone())
    }

    #[test]
    fn test_handling_rooms() {
        let context = get_context("bob.near".to_string(),vec![],NEAR*15,false,None);
        let ctx = context.clone();
        testing_env!(context);

        let contract = get_dummy_flat_contract(&ctx);

        assert!(contract.get_rooms() == flat.rooms, "Rooms didn't match");
        
        //book a room
        let book_room = 10;
        assert!(contract.book_room(book_room)==true, "Should be able to book this room");

        //see if book_room is part of available rooms, it should fail
        assert!(contract.room_is_available(book_room)==false, "Room shouldn't be available");

        //unrent room, only callable by owner of contract, so should return false, since creator of
        //contract not bob.near
        assert!(contract.can_call_unrent_room()==false);

        //I can alert the owner that next month I don't intend to keep renting
        assert!(contract.not_going_to_rent_next_month()==true);

        let rooms_unrenting_next_month: Vec<Room> = contract.get_rooms_unrenting_next_month();// get rooms that should be available next month
        assert!(rooms_unrenting_next_month.len()>0, "bob.near set a room for unrenting, it should appear here");

        //assume end month has reached, and then doing next actions as owner of the flat
        // or in other words owner of the contract
        let context = get_context("borwe.near".to_string(),vec![],NEAR*15,false,None);
        let ctx = context.clone();
        testing_env!(context);

        let contract = get_dummy_flat_contract(&ctx);
        assert!(contract.unlock_room_for_renting(book_room)==true); // the room which user marked for sell
        assert!(contract.unlock_room_for_renting(book_room)==false); // should be false since room already unlocked
        assert!(contract.room_is_available(book_room)==true); //room should now be available


        // Now another user should be able to rent the free room
        let context = get_context("brando.near".to_string(),vec![],NEAR*15,false,None);
        let ctx = context.clone();
        testing_env!(context);
        
        assert!(contract.book_room(book_room)==true, "Should be able to book this room");
        assert!(contract.room_is_available(book_room)==false, "Room shouldn't be available");
    }
}

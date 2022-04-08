use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, Balance, AccountId, Timestamp};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Serialize, Deserialize};
use flats_obj::{House,HouseInfo};

setup_alloc!();

const NEAR: Balance = 1_000_000_000_000_000_000_000_000;

#[derive(Serialize, Deserialize,Clone, BorshSerialize, BorshDeserialize)]
pub struct Payement{
    pub time_of_payment: Timestamp,
    pub book_date: crate::Date,
    pub price: Balance
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct Date{
    pub day: u32,
    pub month: u32,
    pub year: i32
}

impl Date {
    pub fn new(day: u32, month: u32, year: i32)->Self{
        Self{
            day, month, year
        }
    }
}

type BookedDatesByAccounts = LookupMap<Date,AccountId>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HouseContract {
    /// Hold house info
    house: House,
    /// info on the  owner of the house
    owner: AccountId,
    /// Hold payment data
    payments: LookupMap<AccountId, Vec<Payement>>,
    /// See days it has been booked
    booked: BookedDatesByAccounts,
}


impl Default for HouseContract {
  fn default() -> Self {
      panic!("Can't initialize contract from here")
  }
}

#[near_bindgen]
impl HouseContract {
    #[cfg(target_arch = "wasm32")]
    #[init]
    pub fn new(account: AccountId, house: House)-> Self{
        assert!(env::state_exists()==false, "Sorry, can only be called once by contract");
        house.assert_location_valid();
        Self{
            house,
            owner: account,
            booked: BookedDatesByAccounts::new(b"books".to_vec()),
            payments: LookupMap::new(b"payements".to_vec()),
        }
    }



    /// This is only used on tests, ignore this version
    #[cfg(not(target_arch = "wasm32"))]
    #[init]
    pub fn new(account: AccountId, house: House)-> Self{
        house.assert_location_valid();
        Self{
            house,
            owner: account,
            booked: BookedDatesByAccounts::new(b"books".to_vec()),
            payments: LookupMap::new(b"payements".to_vec()),
        }
    }


    /// For verifying if the user is the one who rented
    pub fn verify(&mut self, day: u32, month: u32, year: i32)-> bool{
        let date = Date::new(day,month,year);
        match self.booked.get(&date) {
            Some(x) => {
                if x==env::signer_account_id() {
                    true
                }else{
                    false
                }
            },
            None => false
        }
    }

    /// When new renter wants to rent house
    #[payable]
    pub fn book_house(&mut self, day: u32, month: u32, year: i32)-> bool{
        assert!(year>0, "Year can not be a negative");
        assert!(self.check_date_available(day,month,year)==true, "House isn't available try another date");
        assert!(env::attached_deposit()==self.house.price, 
                "Sorry price for a room is {} NEAR"
                        ,self.house.price/NEAR);
        self.pay_house(day,month,year)
    }

    fn pay_house(&mut self, day: u32, month: u32, year: i32)-> bool{
        let mut payement_from_user: Vec<Payement> = self.payments
            .get(&env::signer_account_id()).unwrap_or(Vec::new());
        let time_of_payment = env::block_timestamp();
        let book_date = crate::Date::new(day,month,year);
        self.booked.insert(&book_date,&env::signer_account_id());
        payement_from_user.push(Payement{
            time_of_payment,
            book_date,
            price: env::attached_deposit()
        });
        //record payment by user
        self.payments.insert(&env::signer_account_id(),&payement_from_user);
        true
    }

    pub fn check_date_available(&self, day: u32, month: u32, year: i32)-> bool{
        let date = crate::Date::new(day,month,year);
        match self.booked.get(&date) {
            Some(_) => false,
            None => true
        }
    }

    pub fn get_owner(&self)-> String{
        self.owner.clone()
    }

    pub fn get_house_info(self)->HouseInfo{
        HouseInfo::new(self.house.clone())
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


    fn get_dummy_flat_contract(ctx: &VMContext)-> HouseContract{

        let name = ctx.current_account_id.clone();
        let price = NEAR*15; // how much it costs to rent a room in the flat
        let location = "-1.227807,36.989969".to_string();
        let features = "Wifi,2 Swimming pools".to_string();
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();
        let house = House::new(name,price,location,
                             features,image);

        // borwe.near is the creator of the contract
        HouseContract::new("borwe.near".to_string(),house)
    }

    #[test]
    fn test_handling_rooms() {
        let context = get_context("bob.near".to_string(),vec![],NEAR*15,false,None);
        let ctx = context.clone();

        testing_env!(ctx);
        let mut contract = get_dummy_flat_contract(&context);
        
        //see if book_room is part of available rooms, it should be true
        assert!(contract.check_date_available(1,1,2022)==true,
            "Room shouldn't be available");

        //book house
        assert!(contract.book_house(1,1,2022)==true,
            "Should be able to book this room");

        //see if book_room is part of available rooms, it should fail
        assert!(contract.check_date_available(1,1,2022)==false,
            "Room shouldn't be available");
    }
}

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{AccountId, Balance, Promise, Gas};
use serde::{Serialize,Deserialize};
use near_sdk::serde_json;
use near_sdk::json_types::U128;
use flats_obj::House;

setup_alloc!();

///for holding a contract address
type Contract = String;

const NEAR: Balance = 1_000_000_000_000_000_000_000_000;
const GAS: Gas = 30_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HouseFactory {
    houses: UnorderedMap<AccountId, UnorderedSet<Contract>>,
    owner: AccountId
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MapHouseContractIdInput{
    house_owner: AccountId,
    house_account: AccountId
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MapHouseContractFlatInput{
    account: AccountId,
    house: House
}

impl Default for HouseFactory {
    fn default() -> Self {
        panic!("Default construction should never happen");
    }
}

#[near_bindgen]
impl HouseFactory {
    #[init]
    pub fn new(owner: AccountId)->Self{
        assert!(env::state_exists()==false, "Sorry state already exists");
        Self{
            houses: UnorderedMap::new(b"houses".to_vec()),
            owner
        }
    }

    #[payable]
    pub fn create_house(&mut self,
        name: String,
        price: U128,
        location: String,
        features: String,
        image: String) -> Promise {

        assert!(env::attached_deposit()==10*NEAR, "Need to send 10 NEAR");
        let user_calling = env::signer_account_id();
        let price: Balance = price.into();
        let house = House::new(name,price,location,features,image);

        let name: String= house.name.clone();
        if name.contains('.'){
            panic!("Name of flat should not contain a '.'");
        }

        assert!(env::is_valid_account_id(name.as_bytes()), 
                "Please pass valid near account name as account name");
        house.assert_location_valid();

        //create house account and push contract
        let mut house_account = house.name.clone();
        assert!(self.check_house_name_available(house_account.clone()),
            "House name already taken, pick another");
        house_account.push_str(".");
        house_account.push_str(env::current_account_id().as_str());
        assert!(env::is_valid_account_id(house_account.clone().as_bytes()),
            "{} is not a valid NEAR account",house_account);

        let input_for_map_house_contract = serde_json::to_vec(&MapHouseContractIdInput{
            house_owner: user_calling,
            house_account: house.name.clone()
        }).unwrap();

        let input_for_house_creation = serde_json::to_vec(
            &MapHouseContractFlatInput{
                account: env::signer_account_id(),
                house 
            }
        ).unwrap();
        Promise::new(house_account.clone()).create_account()
            .transfer(9*NEAR)
            .deploy_contract(
                include_bytes!("../../../out/flats_contract.wasm").to_vec())
            .function_call("new".as_bytes().into(),
                      input_for_house_creation,0,GAS)
            .then(Promise::new(env::current_account_id())
                  .function_call("map_house_contract_to_user_id"
                                 .to_string().into_bytes(),
                 input_for_map_house_contract,0,GAS))
    }

    pub fn map_house_contract_to_user_id(&mut self,house_owner: AccountId
                ,house_account: AccountId)-> String{
        assert!(env::current_account_id()==env::predecessor_account_id(),
            "only contract can call this method");
        match self.houses.get(&house_owner){
            Some(mut houses_owned) => {
                houses_owned.insert(&house_account);
                self.houses.insert(&house_owner,&houses_owned);
            },
            None => {
                let mut houses_owned = 
                    UnorderedSet::new(b"houses_owned".to_vec());
                houses_owned.insert(&house_account);
                self.houses.insert(&house_owner,&houses_owned);
            }
        }
        "DONE".to_string()
    }

    pub fn check_house_name_available(&self, house_name: String)->bool{
        for owner in self.houses.keys(){
            if self.houses.get(&owner).unwrap().contains(&house_name) {
                return false;
            }
        }
        true
    }

    pub fn get_all_houses(&self)-> Vec<Contract>{
        let mut contracts = Vec::new();

        for owner in self.houses.keys(){
            for c in self.houses.get(&owner).unwrap().iter(){
                contracts.push(c.clone());
            }
        }
        contracts
    }

    pub fn get_owner(&self)-> AccountId{
        self.owner.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>,attached_deposit: Balance, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "houses.brian_near".to_string(),
            signer_account_id: "brian_near".to_string(),
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

    #[test]
    fn test_factory_contract_creation() {
        let context = get_context(vec![],0, false);
        let ctx = context.clone();
        testing_env!(ctx);
        let contract = HouseFactory::new(context.signer_account_id.clone());
        assert!(contract.get_owner()==context.signer_account_id.clone(),"signer not equal contract owner");
    }

    #[test]
    fn test_creating_flat_contract() {
        let context = get_context(vec![],10*NEAR, false);
        let ctx = context.clone();
        testing_env!(ctx);
        let mut contract = HouseFactory::new(
            context.signer_account_id.clone());
        //flat contains name,rooms,price(in near) per room,location of building,features(an array
        //string), and an image of the area
        let name = "borwe_towers".to_string();
        let price = NEAR*15; // how much it costs to rent a room in the flat
        let location = "-1.227807,36.989969".to_string();
        let features = "Wifi,2 Swimming pools".to_string();
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();

        contract.create_house(name,U128::from(price),
                 location,features,image);
    }
}

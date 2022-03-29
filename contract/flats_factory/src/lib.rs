use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{AccountId, Balance, Promise, Gas};
use near_sdk::json_types::{U64};
use serde::{Serialize,Deserialize};
use near_sdk::serde_json;

setup_alloc!();

///for holding a contract address
type Contract = String;

const NEAR: Balance = 1_000_000_000_000_000_000_000_000;
const GAS: Gas = 250_000_000_000_000;

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct Position{
    pub latitude: String,
    pub longitude: String
}

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct Flat{
    pub name: String,
    pub rooms: U64,
    pub price: Balance,
    pub location: Position,
    pub features: Option<Vec<String>>,
    pub image: Option<String>
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
pub struct FlatsFactory {
    flats: UnorderedMap<AccountId, UnorderedSet<Contract>>,
    owner: AccountId
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MapFlatContractIdInput{
    flat_owner: AccountId,
    flat_account: AccountId
}

impl Default for FlatsFactory {
    fn default() -> Self {
        panic!("Default construction should never happen");
    }
}

#[near_bindgen]
impl FlatsFactory {
    #[init]
    pub fn new(owner: AccountId)->Self{
        assert!(env::state_exists()==false, "Sorry state already exists");
        Self{
            flats: UnorderedMap::new(b"flats".to_vec()),
            owner
        }
    }

    #[payable]
    pub fn create_flat(&mut self,flat: Flat)-> Promise{
        assert!(env::attached_deposit()==10*NEAR, "Need to send 10 NEAR");
        let user_calling = env::signer_account_id();
        let name: String= flat.name.clone();
        if name.contains('.'){
            panic!("Name of flat should not contain a '.'");
        }
        assert!(env::is_valid_account_id(name.as_bytes()), 
                "Please pass valid near account name as account name");
        assert!(u64::from(flat.rooms)>0, "Can not have a flat with no rooms");
        let _longitude: f64 = flat.location.longitude.clone()
            .parse::<f64>().expect("Longitude didn't contain a float");
        let _latitude: f64 = flat.location.latitude.clone()
            .parse::<f64>().expect("Latitude didn't contain a float value");

        //create flat account and push contract
        let mut flat_account = flat.name.clone();
        assert!(self.check_flat_name_available(flat_account.clone()),
            "Flat name already taken");
        flat_account.push_str(".");
        flat_account.push_str(env::current_account_id().as_str());
        assert!(env::is_valid_account_id(flat_account.clone().as_bytes()),
            "{} is not a valid NEAR account",flat_account);

        let input_for_map_flat_contract = serde_json::to_vec(&MapFlatContractIdInput{
            flat_owner: user_calling,
            flat_account: flat.name.clone()
        }).unwrap();
        Promise::new(flat_account).create_account()
            .transfer(9*NEAR)
            .deploy_contract(
                include_bytes!("../../../out/flats_contract.wasm").to_vec())
            .then(Promise::new(env::current_account_id())
                  .function_call("map_flat_contract_to_user_id"
                                 .to_string().into_bytes(),
                 input_for_map_flat_contract,0,GAS))
    }

    pub fn map_flat_contract_to_user_id(&mut self,flat_owner: AccountId
                ,flat_account: AccountId)-> String{
        assert!(env::current_account_id()==env::predecessor_account_id(),
            "only contract can call this method");
        match self.flats.get(&flat_owner){
            Some(mut flats_owned) => {
                flats_owned.insert(&flat_account);
            },
            None => {
                let mut flats_owned = 
                    UnorderedSet::new(b"flats_owned".to_vec());
                flats_owned.insert(&flat_account);
                self.flats.insert(&flat_owner,&flats_owned);
            }
        }
        "DONE".to_string()
    }

    pub fn check_flat_name_available(&self, flat_name: String)->bool{
        for owner in self.flats.keys(){
            if self.flats.get(&owner).unwrap().contains(&flat_name) {
                return false;
            }
        }
        true
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
            current_account_id: "flats.brian_near".to_string(),
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
        let contract = FlatsFactory::new(context.signer_account_id.clone());
        assert!(contract.get_owner()==context.signer_account_id.clone(),"signer not equal contract owner");
    }

    #[test]
    fn show_flat_structure(){
        let name = "borwe_towers".to_string();
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
        println!("Flat: {:?}",flat);
    }

    #[test]
    fn test_creating_flat_contract() {
        let context = get_context(vec![],10*NEAR, false);
        let ctx = context.clone();
        testing_env!(ctx);
        let mut contract = FlatsFactory::new(
            context.signer_account_id.clone());
        //flat contains name,rooms,price(in near) per room,location of building,features(an array
        //string), and an image of the area
        let name = "borwe_towers".to_string();
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

        contract.create_flat(flat.clone());
    }
}

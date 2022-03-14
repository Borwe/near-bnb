use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::UnorderedMap;
use near_sdk::{AccountId, Balance, Promise};
use near_sdk::json_types::{U64,U128};
use serde::{Serialize,Deserialize};

setup_alloc!();

///for holding a contract address
type Contract = String;

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
    flats: UnorderedMap<AccountId, Contract>,
    owner: AccountId
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
        Self{
            flats: UnorderedMap::new(b"flats".to_vec()),
            owner
        }
    }

    #[payable]
    pub fn create_flat(&mut self,flat: Flat){
        assert!(env::attached_deposit()==10*NEAR, "Need to send 10 NEAR");
        let name: String= flat.name;
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

use serde::{Serialize,Deserialize};
use near_sdk::Balance;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;

/// Represents a house
#[derive(Serialize, Deserialize,Clone, BorshDeserialize, BorshSerialize)]
pub struct House{
    /// Flat name
    pub name: String,
    /// Flat price for renting a room
    pub price: Balance,
    /// Location of the flat
    pub location: String,
    /// Features helpd by the flat
    pub features: Vec<String>,
    /// Image if the flat
    pub image: String
}


/// Represents a house info, as json
#[derive(Serialize, Deserialize,Clone, BorshDeserialize, BorshSerialize)]
pub struct HouseInfo{
    /// Flat name
    pub name: String,
    /// Flat price for renting a room
    pub price: U128,
    /// Location of the flat
    pub location: String,
    /// Features helpd by the flat
    pub features: Vec<String>,
    /// Image if the flat
    pub image: String
}

impl HouseInfo{
    pub fn new(house: House)->Self{
        Self{
            name: house.name,
            price: U128::from(house.price),
            location: house.location,
            features: house.features,
            image: house.image
        }
    }
}

impl House{
    pub fn new(name: String, price: Balance,location: String,
            features: String, image: String)->Self{

        assert!(location.clone().split(',').into_iter().count()==2,
            "Sorry, not valid with latitude and longitude");

        
        assert!(name.len()>0, "Name can't be empt");
        assert!(price>0, "Price can not be free");
        Self{
            name,
            price,
            location,
            features: features.split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            image
        }
    }

    pub fn assert_location_valid(&self){
        assert!(self.location.clone().split(',').into_iter().count()==2,
            "Sorry, not valid with latitude and longitude");
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    const NEAR: Balance = 1_000_000_000_000_000_000_000_000;

    #[test]
    fn test_flat_creation(){
        
        let name = "borwe_towers".to_string();
        let price = NEAR*15; // how much it costs to rent a room in the flat
        let location = "1.000,1.000";
        let features = "Wifi, 2 Swimming pools".to_string();
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();

        let _flat = House::new(name,price,
              location.to_string(), features,image);

    }

    #[test]
    #[should_panic]
    fn test_flat_creation_failure(){

        let name = "borwe_towers".to_string();
        let price = NEAR*15; // how much it costs to rent a room in the flat
        let location = "1.000,1.000,1.000";
        let features = "Wifi,2 Swimming pools".to_string();
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();

        let _flat = House::new(name,price,
              location.to_string(), features,image);
    }
}

use serde::{Serialize,Deserialize};
use near_sdk::Balance;
use near_sdk::json_types::U64;

/// Represents a flat
#[derive(Serialize, Deserialize,Clone)]
pub struct Flat{
    /// Flat name
    pub name: String,
    /// Flat number of rooms
    pub rooms: U64,
    /// Flat price for renting a room
    pub price: Balance,
    /// Location of the flat
    pub location: String,
    /// Features helpd by the flat
    pub features: Option<Vec<String>>,
    /// Image if the flat
    pub image: Option<String>
}

impl Flat{
    pub fn new(name: String, rooms: U64, price: Balance,location: String,
            features: Option<Vec<String>>, image: Option<String>)->Self{
        assert!(location.clone().split(',').into_iter().count()==2,
            "Sorry, not valid with latitude and longitude");
        Self{
            name,
            rooms,
            price,
            location,
            features,
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
        let rooms = U64::from(300);
        let price = NEAR*15; // how much it costs to rent a room in the flat
        let location = "1.000,1.000";
        let features = vec!["Wifi".to_string(),
            "2 Swimming pools".to_string(),
            "Big open compound".to_string(),
            "alot of greenarie".to_string()];
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();

        let _flat = Flat::new(name,rooms,price,
              location.to_string(), Some(features),Some(image));

    }

    #[test]
    #[should_panic]
    fn test_flat_creation_failure(){

        let name = "borwe_towers".to_string();
        let rooms = U64::from(300);
        let price = NEAR*15; // how much it costs to rent a room in the flat
        let location = "1.000,1.000,1.000";
        let features = vec!["Wifi".to_string(),
            "2 Swimming pools".to_string(),
            "Big open compound".to_string(),
            "alot of greenarie".to_string()];
        let image = "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/1c/d3/c1/64/exterior.jpg?w=800&h=-1&s=1".to_string();

        let _flat = Flat::new(name,rooms,price,
              location.to_string(), Some(features),Some(image));
    }
}

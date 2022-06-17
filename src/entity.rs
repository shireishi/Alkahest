
use super::{
    game::Location,
    inventory::Item
};

use std::{
    io, fmt
};

pub struct Entity<T> {
    pub entity: Option<T>,
    pub location: Location,
    
    pub health: f32,
    pub stamina: f32,
    pub hunger: f32,
    pub mana: f32
} impl<T> Entity<T> {
    pub fn new(entity: Option<T>) -> Self {
        Self {
            entity,
            location: Location::new(),

            health: 100.,
            stamina: 100.,
            hunger: 100.,
            mana: 0.
        }
    }
}

pub struct Player {
    inventory: Vec<Item>
} impl Player {
    pub fn new(inventory: Vec<Item>) -> Player {
        Player {
            inventory
        }
    }
} impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list_string: String = "".to_string();

        for item in self.inventory.iter() {
            list_string += &format!("{} : {} : {:?}", item.item_name, item.item_id, item.attributes);
            list_string += "\n";
        }

        write!(f, "{}", list_string)
    }
} impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{}", self)
    }
}

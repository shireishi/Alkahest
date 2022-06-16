
use super::{
    game::Location,
    inventory::Item
};

pub struct Entity {
    location: Location,
    
    health: f32,
    stamina: f32,
    hunger: f32,
    mana: f32
}

pub struct Player {
    inventory: Vec<Item>
}

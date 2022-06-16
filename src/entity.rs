
use super::{
    game::Location,
    inventory::*
};

pub struct Entity {
    location: Location,
    
    health: f32,
    stamina: f32,
    hunger: f32,
    mana: f32
}

pub struct Player {
    inventory: Vec<inventory::Item>
}

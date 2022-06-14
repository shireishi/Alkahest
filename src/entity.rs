use bevy::{
    prelude::*
};

use super::{
    Location,
    Player
};

#[derive(Component, PartialEq, Eq)]
pub enum EntityType {
    Unknown,

    Player(Player),
    Mob,

    Block,
    Structure
}
impl Default for EntityType {
    fn default() -> Self { EntityType::Unknown }
}

#[derive(Default, Component)]
pub struct GameEntity {
    pub entity_type: EntityType,

    pub handle: Handle<Image>,

    pub height: f32,
    pub width: f32,
    pub position: Location,
    pub dimensions: [u32; 2],


}
impl GameEntity {
    pub fn new(hand: Handle<Image>) -> GameEntity {
        GameEntity {
            entity_type: EntityType::Unknown,
            handle: hand,
            position: Location::new(),
            ..default()
        }
    }
}
use bevy::{
    prelude::*
};

use super::{
    Location
};

#[derive(Default, Component)]
pub struct GameEntity {
    pub handle: Handle<Image>,
    pub height: f32,
    pub width: f32,
    pub position: Location
}
impl GameEntity {
    pub fn new(hand: Handle<Image>) -> GameEntity {
        GameEntity {
            handle: hand,
            position: Location::new(),
            ..default()
        }
    }
}
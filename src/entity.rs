<<<<<<< HEAD
use bevy::{
    prelude::*
};

use super::{
    Location,
    Player
};

#[derive(Debug, Clone, Component)]
pub struct GameEntity<T: Default> {
    pub handle: Handle<Image>,
    pub entity: Option<Entity>,
    pub game_entity: Option<T>,
    
} impl<T: Default> GameEntity<T> {
    pub fn new(
        game_entity: Option<T>
    ) -> GameEntity<T> {
        GameEntity {
            game_entity,
            ..default()
        }
    }

    pub fn add_handle(
        &mut self,
        hand: Handle<Image>
    ) -> &mut Self {
        self.handle = hand;
        self
    }
} impl<T: Default> Default for GameEntity<T> {
    fn default() -> Self {
        Self {
            game_entity: Some(T::default()),
            ..default()
        }
    }
}


// #[derive(Default, Component)]
// pub struct GameEntity {
//     pub handle: Handle<Image>,
//     pub height: f32,
//     pub width: f32,
//     pub position: Location
// }
// impl GameEntity {
//     pub fn new(hand: Handle<Image>) -> GameEntity {
//         GameEntity {
//             handle: hand,
//             position: Location::new(),
//             ..default()
//         }
//     }
// }
=======
>>>>>>> 844bf1229fee29295b99707076265aad6a5332e9

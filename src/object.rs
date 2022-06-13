use bevy::{
    prelude::*,
    asset::Handle,
    ecs::system::EntityCommands
};

use super::{
    Location
};

// ? Figure out how to have the GameObject struct take in an EntityCommands //
// ? struct as the object parameter //

#[derive(Default, Component)]
pub struct GameObject {
    id: u32,
    location: Location,
    dimensions: Vec2
}

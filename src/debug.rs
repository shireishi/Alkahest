use bevy::{
    prelude::*,
};

use super::{
    GameEntity,
    Player,
};

pub fn player_tracker(
    player: Res<Player>
) {
    info!("Player Location: X: {} Y: {} Z: {}", player.location.x, player.location.y, player.location.z);
}

pub fn draw_collision(
    mut objects: Query<&mut GameEntity>
) {
    for _object in objects.iter_mut() {
        // draw lines on bounds of collision mesh
    }
}
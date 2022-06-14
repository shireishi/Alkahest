use bevy::{
    prelude::*,
};

use super::{
    GameEntity,
    EntityType
};

pub fn player_tracker(
    mut entities: Query<&mut GameEntity>
) {
    for ent in entities.iter_mut() {
        if ent.entity_type == EntityType::PlayerType {
            info!("Player Location: X: {} Y: {} Z: {}", ent.position.x, ent.position.y, ent.position.z);
        }
    }
}

pub fn draw_collision(
    mut objects: Query<&mut GameEntity>
) {
    for _object in objects.iter_mut() {
        // draw lines on bounds of collision mesh
    }
}
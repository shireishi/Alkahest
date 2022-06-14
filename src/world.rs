use bevy::{
    asset::Handle,
    ecs::system::EntityCommands
};

use super::{
    Mesh,
    default,
    Vec3,
    GameEntity,
    EntityType
};

struct TextureMap {
    texture: String,
    color: Vec3,
    height: f32
}

#[derive(Default)]
pub struct Game {
    pub world_mesh: Vec<Vec<f32>>,

    pub height: f32,
    pub width: f32,

    pub players: Vec<GameEntity>
}
impl Game {
    pub fn new(
        h: f32,
        w: f32
    ) -> Game {
        Game {
            height: h,
            width: w,
            ..default()
        }
    }
    pub fn add_object(
        &mut self,
        _object: EntityCommands,
        _pos_x: f32,
        _pos_y: f32
    ) {
        return;
    }
}
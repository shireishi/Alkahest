use bevy::{
    prelude::*, ecs::query::WorldQuery
};

use crate::world::Game;

use super::{
    Location,
    invs
};

#[derive(Component, PartialEq, Eq, Clone, Debug, Copy)]
pub enum EntityType {
    Unknown,

    PlayerType,
    MobType,

    BlockType,
    StructureType
}
impl Default for EntityType {
    fn default() -> Self { EntityType::Unknown }
}

#[derive(Default, Component, Clone, Debug, PartialEq, Eq)]
pub struct GameEntity {
    pub entity_type: EntityType,

    pub handle: Handle<Image>,
    pub entity: Option<Entity>,
    pub inventory: Vec<invs::Item>,


    pub height: u32,
    pub width: u32,
    pub position: Location,

    pub health: u32,
    pub stamina: u32,
    pub hunger: u32,
    pub mana: u32
}
impl GameEntity {
    pub fn new(ent: EntityType) -> GameEntity {
        GameEntity { 
            entity_type: ent,
            ..default()}
    }
    pub fn add_inventory(&mut self, inv: Vec<invs::Item>) {
        self.inventory = inv;
    }
}
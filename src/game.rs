#![allow(dead_code)]

use super::{
    entity::Entity,
    entity::{
        Player, Mob, Hostile, Unknown
    },
    fmt,
    any::{
        Any, TypeId
    }
};

pub const WORLD_X: usize = 27;
pub const WORLD_Y: usize = 27;

pub enum GameState {
    BackgroundInit,
    WorldInit,
    Menu,
    Playing,
    Paused,
    End
}

pub struct World {
    pub players: Vec<Entity<Player>>,
    pub mobs: Vec<Entity<Mob>>,
    pub hostiles: Vec<Entity<Hostile>>,
    pub unknown_entities: Vec<Entity<Unknown>>,

    pub mesh: Vec<Vec<u32>>
} impl World {
    pub fn new() -> World {
        World {
            players: Vec::new(),
            mobs: Vec::new(),
            hostiles: Vec::new(),
            unknown_entities: Vec::new(),

            mesh: Vec::new()
        }
    }

    // insert structs into the entities vector for World
    pub fn insert(&mut self, ent: &dyn Any) {
        if ent.is::<Entity<Player>>() {
            println!("[Insert] Entity is player!");
            self.players.push(ent.downcast_ref::<Entity<Player>>().unwrap().clone());
            ()
        }
        else if ent.is::<Entity<Mob>>() {
            println!("[Insert] Entity is mob!");
            self.mobs.push(ent.downcast_ref::<Entity<Mob>>().unwrap().clone());
            ()
        }
        else if ent.is::<Entity<Hostile>>() {
            println!("[Insert] Entity is hostile mob");
            self.hostiles.push(ent.downcast_ref::<Entity<Hostile>>().unwrap().clone());
            ()
        }
        else {
            println!("[Insert] Failed to find the entity type");
        }
    }

    // return an array of T from the entities vector in World
    pub fn grab<T: fmt::Debug + 'static>(&mut self, _ent: T) {
        ()
    }

    pub fn component_update(&mut self) {
        for player in self.players.iter_mut() {
            println!("[Comp] {:?}", player.entity.as_ref().unwrap());
        }

        for mob in self.mobs.iter_mut() {
            println!("[Comp] {:?}", mob.entity.as_ref().unwrap());
        }

        for hostile in self.hostiles.iter_mut() {
            println!("[Comp] {:?}", hostile.entity.as_ref().unwrap());
        }
    }
}

#[derive(Clone, Copy)]
pub struct Location {
    x: f32,
    y: f32,
    z: f32
} impl Location {
    pub fn new() -> Location {
        Location {
            x: 0.,
            y: 0.,
            z: 0.
        }
    }
}

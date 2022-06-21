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
    pub systems: Option<Vec<Box<dyn Fn(&World)>>>,


    pub mesh: Vec<Vec<u32>>
} impl World {
    pub fn new() -> World {
        World {
            players: Vec::new(),
            mobs: Vec::new(),
            hostiles: Vec::new(),
            unknown_entities: Vec::new(),
            systems: Some(Vec::new()),

            mesh: Vec::new()
        }
    }

    // insert structs into the entities vector for World
    pub fn insert(&mut self, ent: &dyn Any) {
        if ent.is::<Entity<Player>>() {
            println!("[Insert] Player entity was created!");
            self.players.push(ent.downcast_ref::<Entity<Player>>().unwrap().clone());
            ()
        }
        else if ent.is::<Entity<Mob>>() {
            println!("[Insert] Mob entity was created!");
            self.mobs.push(ent.downcast_ref::<Entity<Mob>>().unwrap().clone());
            ()
        }
        else if ent.is::<Entity<Hostile>>() {
            println!("[Insert] Hostile mob entity was created!");
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
            println!("[Comp] Components: {:?}", player);
        }

        for mob in self.mobs.iter_mut() {
            println!("[Comp] {:?}", mob.entity.as_ref().unwrap());
            println!("[Comp] Components: {:?}", mob);
        }

        for hostile in self.hostiles.iter_mut() {
            println!("[Comp] {:?}", hostile.entity.as_ref().unwrap());
            println!("[Comp] Components: {:?}", hostile);
        }
    }

    pub fn add_system(&mut self, sys: &'static dyn Fn(&World)) {
        self.systems.as_mut().unwrap().push(Box::new(sys));
    }

    fn unbox<T>(value: Box<T>) -> T {
        *value
    }

    pub fn systems_update(&self) {
        for system in self.systems.as_ref().unwrap() {
            // system = self.unbox::<system>();
            system(self);
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

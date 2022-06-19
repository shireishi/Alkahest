#![allow(non_snake_case)]
#![allow(unused_imports)]

mod debug;
mod entity;
mod game;
mod inventory;

use std::{
    io, fmt, any
};

use entity::*;
use debug::*;
use game::*;
use inventory::*;

fn update(mut world: World) { 
    world.component_update();
}

fn main() {
    let mut inventory: Vec<Item> = Vec::with_capacity(27usize);
    inventory.push(Item::new("Diamond Sword".to_string(), vec![ItemAttributes::Sword]));
    inventory.push(Item::new("Stone".to_string(), vec![ItemAttributes::Misc]));

    let player = Entity::new(Some(Player::new("Keyshin_".to_string(), inventory)));
    let _mob = Entity::new(Some(Mob::new()));
    let _hostile = Entity::new(Some(Hostile::new()));

    let mut world: World = World::new();

    world.insert(&player);
    // world.insert(&mob);
    // world.insert(&hostile);

    update(setup(world));
}

fn setup(mut world: World) -> World {
    for _ in 0 .. WORLD_Y {
            world.mesh.push(Vec::with_capacity(WORLD_X));
    }

    world
}

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
    world.systems_update();
}

pub fn test(world: &World) {
    println!("{:?}", world.mobs);
    println!("Hello World from the test function!");
}

fn main() {
    let mut inventory: Vec<Item> = Vec::with_capacity(27usize);
    inventory.push(Item::new(
        "Diamond Sword".to_string(),
        vec![ItemAttributes::Sword])
    );

    let mut player = Entity::new(Some(Player::new("Keyshin_".to_string(), inventory)));
    player.add_component(vec![Components::KeyboardInput, Components::Collision]);

    let stalker = Entity::new(Some(Mob::new()));

    let mut world: World = World::new();

    world.insert(&player);
    world.insert(&stalker);
    world.add_system(&test);

    update(setup(world));
}

fn setup(mut world: World) -> World {
    for _ in 0 .. WORLD_Y {
            world.mesh.push(Vec::with_capacity(WORLD_X));
    }

    world
}

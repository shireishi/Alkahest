#![allow(non_snake_case)]
#![allow(unused_imports)]

mod debug;
mod entity;
mod game;
mod inventory;

use std::{
    io, fmt
};


use entity::*;
use debug::*;
use game::*;
use inventory::*;

fn main() {
    let mut inventory: Vec<Item> = Vec::with_capacity(27usize);
    inventory.push(Item::new("Diamond Sword".to_string(), vec![ItemAttributes::Sword]));
    inventory.push(Item::new("Stone".to_string(), vec![ItemAttributes::Misc]));

    let player: Entity<Player> = Entity::new(Some(Player::new(inventory)));
    println!("{:?}", player.entity.unwrap());
}

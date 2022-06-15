#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{
    io, fmt, borrow::Borrow
};

mod world;

use world::*;

struct Health(u32);
struct Name(&'static str);

fn main() {
    let mut world: World = World::new();
    let player: usize = world.new_entity();

    world.add_comp(player, Health(100));
    world.add_comp(player, Name("Helena"));

    let mut health = world.borrow::<Health>().unwrap();
    let mut name = world.borrow::<Name>().unwrap();

    let zip = health.iter_mut().zip(name.iter_mut());
    let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

    for (health, name) in iter {
        if health.0 <= 0 {
            println!("{} has perished.", name.0);
        } else if health.0 >= 50 {
            println!("{} is prospering", name.0);
        }
    }
}

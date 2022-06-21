#![allow(dead_code)]

use super::{
    game::Location,
    inventory::Item
};

use std::{
    io, fmt
};

#[derive(Clone, Copy, Debug)]
pub enum Components {
    KeyboardInput,
    HostileAI,
    IdleAI,
    NeutralAI,
    PeacefulAI,
    Collision
}

#[derive(Clone)]
pub struct Entity<T> {
    pub entity: Option<T>,
    pub location: Location,
    pub components: Vec<Components>,
    
    pub health: f32,
    pub stamina: f32,
    pub hunger: f32,
    pub mana: f32
} impl<T> Entity<T> {
    pub fn new(entity: Option<T>) -> Self {
        Self {
            entity,
            location: Location::new(),
            components: Vec::new(),

            health: 100.,
            stamina: 100.,
            hunger: 100.,
            mana: 0.
        }
    }
    pub fn add_component(&mut self, components: Vec<Components>) {
        for i in 0 .. components.len() {
            self.components.push(components[i]);
        }
    }
}
impl fmt::Display for Entity<Player> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
impl fmt::Debug for Entity<Player> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted_string = "".to_string();
        for component in self.components.iter() {
            formatted_string += &format!("{:?}\n", component);
        }

        write!(f, "{}", formatted_string)
    }
}
impl fmt::Display for Entity<Mob> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
impl fmt::Debug for Entity<Mob> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted_string = "".to_string();
        for component in self.components.iter() {
            formatted_string += &format!("{:?}\n", component);
        }

        write!(f, "{}", formatted_string)
    }
}
impl fmt::Display for Entity<Hostile> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
impl fmt::Debug for Entity<Hostile> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted_string = "".to_string();
        for component in self.components.iter() {
            formatted_string += &format!("{:?}\n", component);
        }

        write!(f, "{}", formatted_string)
    }
}

#[derive(Clone)]
pub struct Player {
    name: String,
    inventory: Vec<Item>
} impl Player {
    pub fn new(name: String, inventory: Vec<Item>) -> Player {
        Player {
            name,
            inventory
        }
    }
} impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list_string: String = "".to_string();
        let player_name: String = format!("Player name: {}\nInventory:\n", self.name);

        list_string += &player_name;

        for item in self.inventory.iter() {
            list_string += &format!("    name: {}  id: {} attrs: {:?}\n", item.name, item.id, item.attributes);
        }

        write!(f, "{}", list_string)
    }
} impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{}", self)
    }
}

#[derive(Clone)]

pub struct Mob;
impl Mob {
    pub fn new() -> Mob {
        Mob{}
    }
} impl fmt::Display for Mob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_string: String = "Mob Display".to_string();

        write!(f, "{}", formatted_string)
    }
} impl fmt::Debug for Mob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_string: String = "Mob Debug".to_string();

        write!(f, "{}", formatted_string)
    }
}

#[derive(Clone)]

pub struct Hostile;
impl Hostile {
    pub fn new() -> Hostile{
        Hostile{}
    }
} impl fmt::Display for Hostile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_string: String = "Hostile Mob Display".to_string();

        write!(f, "{}", formatted_string)
    }
} impl fmt::Debug for Hostile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_string: String = "Hostile Mob Debug".to_string();

        write!(f, "{}", formatted_string)
    }
}

pub struct Unknown;
pub trait GameEntity{
    fn test();
}

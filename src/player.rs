use super::{
    Location,
    invs,
    Timer,
    default,
    Component,
    Handle,
    Image,
    AssetPath,
    Asset,
    Entity
};

#[derive(Default, Debug, Clone, Component, PartialEq, Eq)]
pub struct Player {
    // Game Values //
    pub location: Location,
    pub entity: Option<Entity>,
    pub handle: Handle<Image>,
    pub dimensions: [u32; 2],
    
    // Player Status //
    pub health: u32,
    pub stamina: u32,
    pub hunger: u32,
    pub mana: u32,

    // Player Storage //
    pub inventory: Vec<invs::Item>,
} impl Player {
    pub fn new(inv: Vec<invs::Item>) -> Player {
        Player {
            location: Location {
                ..default()
            },
            inventory: inv,
            // jump_cooldown: Timer::from_seconds(2., true),
            ..default()
        }
    }
    pub fn add_handle(&mut self, hand: Handle<Image>) {
        self.handle = hand;
    }
    // impl Default for Player {
    //     fn default() -> Self {
    //         Player {
    //             location: Location::new(),
    //             health: 100.,
    //             stamina: 100.,
    //             hunger: 100.,
    //             mana: 100.,
    //             ..default()
    //         }
    //     }
    // }
}
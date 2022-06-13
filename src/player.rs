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

#[derive(Default, Debug, Clone, Component)]
pub struct Player {
    // Game Values //
    pub location: Location,
    pub entity: Option<Entity>,
    pub handle: Handle<Image>,
    
    // Player Status //
    pub health: f32,
    pub stamina: f32,
    pub hunger: f32,
    pub mana: f32,

    // Player Storage //
    pub inventory: Vec<invs::Item>,

    // Countdowns //
    jump_cooldown: Timer
} impl Player {
    pub fn new(inv: Vec<invs::Item>) -> Player {
        Player {
            location: Location {
                ..default()
            },
            inventory: inv,
            jump_cooldown: Timer::from_seconds(2., true),
            ..default()
        }
    }
    pub fn add_handle(&mut self, hand: Handle<Image>) {
        self.handle = hand;
    }
}
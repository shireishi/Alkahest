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
    // Player Status //
    pub health: f32,
    pub stamina: f32,
    pub hunger: f32,
    pub mana: f32,

    pub location: Location,

    // Player Storage //
    pub inventory: Vec<invs::Item>,
} impl Player {
    pub fn new(inv: Vec<invs::Item>) -> Player {
        Player {
            inventory: inv,
            ..default()
        }
    }
    pub fn add_handle(&mut self, hand: Handle<Image>) {
        self.handle = hand;
    }
}
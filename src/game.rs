#![allow(dead_code)]
pub enum GameState {
    BackgroundInit,
    WorldInit,
    Menu,
    Playing,
    Paused,
    End
}

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

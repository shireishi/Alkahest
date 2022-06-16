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
}

use super::{
    Component
};

#[derive(Clone, Copy, Debug, Eq, Component, PartialEq, Hash)]
pub enum GameState {
    BackgroundInit,
    WorldInit,
    Menu,
    Playing,
    Paused,
    End
}
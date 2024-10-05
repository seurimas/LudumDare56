use bevy::prelude::States;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Loading,
    Playing,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

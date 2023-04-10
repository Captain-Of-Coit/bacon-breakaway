use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    GameOver,
    CornStore,
    Finished,
    Credits,
}

// Loading -> MainMenu
// MainMenu -> InGame
// InGame -> GameOver
// GameOver -> CornStore
// CornStore -> InGame

// InGame -> Finished
// Finished -> Credits
// Credits -> MainMenu

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<GameState>();
    }
}
use bevy::prelude::*;

/// The main game states for SettleTown. These are states that the application itself
/// can be in.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The game is loading and the `SettleTownLoadingSplashScreen` is being displayed.
    #[default]
    LoadingSplashScreen,
    // MainMenu,
    InGameWorld,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingState {
    #[default]
    CreatingGameWorld,
}

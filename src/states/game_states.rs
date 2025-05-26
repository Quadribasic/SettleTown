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

/// The loading states for SettleTown. All of these should be completed before
/// the `GameState::InGameWorld` state is entered.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingState {
    /// The loading splash screen is being initialized and shown.
    #[default]
    IntializingLoadingSplashScreen,
    /// The game world is being created.
    CreatingGameWorld,
    /// The sprites are being loaded.
    LoadingSprites,
    /// An idle state that gives homage back to Sim City.
    ReticulatingSplines,
}

impl LoadingState {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingState::IntializingLoadingSplashScreen => "Initializing loading splash screen...",
            LoadingState::CreatingGameWorld => "Creating game world...",
            LoadingState::LoadingSprites => "Loading sprites...",
            LoadingState::ReticulatingSplines => "Reticulating splines...",
        }
    }
}

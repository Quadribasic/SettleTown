/// The main game states for SettleTown. These are states that the application itself
/// can be in.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStates {
    /// The game is loading and the `SettleTownLoadingSplashScreen` is being displayed.
    LoadingSplashScreen,
    // MainMenu,
    InGameWorld,
}

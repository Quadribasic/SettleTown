use std::time::Duration;

use bevy::{prelude::*, window::WindowPlugin};
use plugins::{
    game_world_window::{
        GameWorldWindow, SettleTownGameWorldWindowPlugin, DEFAULT_WINDOW_HEIGHT,
        DEFAULT_WINDOW_WIDTH,
    },
    splash_screen::{SettleTownLoadingSplashScreenPlugin, SplashScreenWindow},
};
use states::game_states::GameState;

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

#[derive(Component)]
pub struct DefaultWindow;

struct DurationSplashScreenVisible(Duration);

impl Default for DurationSplashScreenVisible {
    fn default() -> Self {
        DurationSplashScreenVisible(Duration::from_secs(5))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        // Add the marker component to the default window.
        .add_systems(
            Startup,
            |mut commands: Commands, window: Query<Entity, Added<Window>>| {
                if let Ok(window_entity) = window.get_single() {
                    commands.entity(window_entity).insert(DefaultWindow);
                }
            },
        )
        .add_plugins(SettleTownLoadingSplashScreenPlugin)
        .add_systems(
            Update,
            transition_from_loading_splash_screen_once_time_has_elapsed
                .run_if(in_state(GameState::LoadingSplashScreen)),
        )
        .add_systems(
            OnExit(GameState::LoadingSplashScreen),
            make_main_window_invisible,
        )
        .add_plugins(SettleTownGameWorldWindowPlugin {
            enabled_state: GameState::InGameWorld,
            _is_fullscreen: false,
            window_width: DEFAULT_WINDOW_WIDTH as i32,
            window_height: DEFAULT_WINDOW_HEIGHT as i32,
        })
        .run();
}

fn transition_from_loading_splash_screen_once_time_has_elapsed(
    length: Local<DurationSplashScreenVisible>,
    time: Res<Time<Fixed>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if time.elapsed_seconds() >= length.0.as_secs() as f32
        && *state.get() == GameState::LoadingSplashScreen
    {
        info!(
            "{:?} seconds have elapsed. Transitioning states...",
            length.0.as_secs()
        );
        next_state.set(GameState::InGameWorld);
    }
}

fn make_main_window_invisible(
    mut commands: Commands,
    default_window: Query<Entity, With<DefaultWindow>>,
    splash_screen_window: Query<Entity, With<SplashScreenWindow>>,
) {
    // Create the game world window first
    info!("Creating game world window");
    commands.spawn((
        Window {
            title: "SettleTown".into(),
            resolution: (1408.0, 768.0).into(),
            resizable: false,
            ..default()
        },
        GameWorldWindow,
    ));

    // Then despawn the default window
    info!("Despawning default window");
    commands.entity(default_window.single()).despawn();

    // Finally despawn the splash screen window
    let window_entity = splash_screen_window.single();
    info!(
        "Despawning splash screen window (Entity: {:?})",
        window_entity
    );
    commands.entity(window_entity).despawn();
}

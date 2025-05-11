use std::time::Duration;

use bevy::{prelude::*, window::WindowPlugin};
use clap::Parser;
use plugins::{
    game_world_window::{
        GameWorldWindow, SettleTownGameWorldWindowPlugin, DEFAULT_WINDOW_HEIGHT,
        DEFAULT_WINDOW_WIDTH,
    },
    splash_screen::{SettleTownLoadingSplashScreenPlugin, SplashScreenWindow},
};
use resources::config::SettleTownConfig;
use states::game_states::GameState;

#[derive(Parser, Debug)]
#[command(
    author = "Quadribasic Companies, LLC",
    version = "0.1.0",
    about = "SettleTown",
    long_about = "SettleTown is a game about settling a town based in the mythological American \
                  Old West. It is a work in progress and not all features are available yet."
)]
struct Args {
    /// Disable the loading splash screen
    #[arg(long)]
    disable_splash_screen: bool,

    /// Add the base configuration file (if it hasn't been added already)
    /// and exit.
    #[arg(long)]
    create_config_file: bool,

    /// Add the base configuration file, overwriting it if it exists.
    #[arg(long)]
    overwrite_config_file_with_default: bool,
}

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
    let args = Args::parse();

    // Create the app and and default plugins before all else because this sets
    // up logging and other things that are necessary for the rest of the application.
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            visible: false,
            ..default()
        }),
        ..default()
    }));

    let mut configuration;
    if args.overwrite_config_file_with_default {
        configuration = SettleTownConfig::clobber_from_default();
    } else {
        configuration = SettleTownConfig::create_from_default();
    }

    if !args.create_config_file && !args.overwrite_config_file_with_default {
        // Add the marker component to the default window.
        app.add_systems(Startup, label_default_window);

        if !args.disable_splash_screen {
            app.add_plugins(SettleTownLoadingSplashScreenPlugin)
                .add_systems(
                    Update,
                    transition_from_loading_splash_screen_once_time_has_elapsed
                        .run_if(in_state(GameState::LoadingSplashScreen)),
                )
                .add_systems(
                    OnExit(GameState::LoadingSplashScreen),
                    make_default_window_invisible,
                );
        } else {
            app.insert_state(GameState::InGameWorld);
            app.add_systems(
                Startup,
                make_default_window_invisible.after(label_default_window),
            );
        }

        app.add_plugins(SettleTownGameWorldWindowPlugin {
            enabled_state: GameState::InGameWorld,
            _is_fullscreen: false,
            window_width: DEFAULT_WINDOW_WIDTH as i32,
            window_height: DEFAULT_WINDOW_HEIGHT as i32,
        });
        app.run();
    }
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

fn label_default_window(mut commands: Commands, window: Query<Entity, Added<Window>>) {
    if let Ok(window_entity) = window.get_single() {
        commands.entity(window_entity).insert(DefaultWindow);
    }
}

fn make_default_window_invisible(
    mut commands: Commands,
    default_window: Query<Entity, With<DefaultWindow>>,
) {
    // Then despawn the default window
    info!("Despawning default window");
    if let Ok(default_window_entity) = default_window.get_single() {
        commands.entity(default_window_entity).despawn();
    } else {
        info!("No default window was detected, skipping despawn")
    }
}

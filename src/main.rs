use std::time::Duration;

use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
    window::WindowPlugin,
};
use clap::{command, Parser};
use plugins::{
    game_world_window::{
        SettleTownGameWorldWindowPlugin, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH,
    },
    layer::{ComputeLayerTask, EdgeDirection, GroundLayerBuilder, GroundTileType, LayerStack},
    splash_screen::SettleTownLoadingSplashScreenPlugin,
};
use resources::config::SettleTownConfig;
use states::game_states::{GameState, LoadingState};

#[derive(Parser, Debug)]
#[command(
    author = "Quadribasic Companies, LLC",
    version = "0.1.0",
    about = "SettleTown",
    long_about = "SettleTown is a game about settling a town based in the mythological American \
                  Old West. It is a work in progress and not all features are available yet."
)]
struct Args {
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

/// The minimum duration that the splash screen should be visible. If the loading of
/// the game takes longer than this, then the splash screen will be displayed for as
/// long as it takes to load. Otherwise, the splash screen will be displayed for the
/// length of time specified here.
struct DurationSplashScreenVisible(Duration);

impl Default for DurationSplashScreenVisible {
    fn default() -> Self {
        DurationSplashScreenVisible(Duration::from_secs(5))
    }
}

#[derive(Debug, Event)]
pub struct GameWorldCreatedEvent;

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

    let configuration = if args.overwrite_config_file_with_default {
        SettleTownConfig::clobber_from_default()
    } else {
        SettleTownConfig::create_from_default()
    };

    // Insert the configuration resource into the app
    app.insert_resource(configuration);

    // This just controls whether we want to exit after clobbering or initializing the configuration
    // file.
    if !args.create_config_file && !args.overwrite_config_file_with_default {
        // Add the marker component to the default window.
        app.add_systems(Startup, label_default_window);

        app.add_plugins(SettleTownLoadingSplashScreenPlugin);
        app.add_systems(
            Update,
            transition_from_loading_splash_screen_once_time_has_elapsed.run_if(
                in_state(GameState::LoadingSplashScreen)
                    .and(in_state(LoadingState::ReticulatingSplines)),
            ),
        );
        app.add_systems(
            OnExit(GameState::LoadingSplashScreen),
            make_default_window_invisible,
        );

        app.add_systems(
            OnEnter(LoadingState::IntializingLoadingSplashScreen),
            create_game_world,
        );
        app.add_systems(
            Update,
            poll_layer_stack_task.run_if(in_state(LoadingState::CreatingGameWorld)),
        );

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
    if time.elapsed_secs() >= length.0.as_secs() as f32
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
    if let Ok(window_entity) = window.single() {
        commands.entity(window_entity).insert(DefaultWindow);
    }
}

fn make_default_window_invisible(
    mut commands: Commands,
    default_window: Query<Entity, With<DefaultWindow>>,
) {
    // Then despawn the default window
    info!("Despawning default window");
    if let Ok(default_window_entity) = default_window.single() {
        commands.entity(default_window_entity).despawn();
    } else {
        info!("No default window was detected, skipping despawn")
    }
}

fn create_game_world(mut commands: Commands, mut next_state: ResMut<NextState<LoadingState>>) {
    // Note that this needs to happen in a separate thread because it takes a long time and
    // we don't want to block the main thread.
    let thread_pool = AsyncComputeTaskPool::get();

    // Spawn new task on the AsyncComputeTaskPool; the task will be
    // executed in the background, and the Task future returned by
    // spawn() can be used to poll for the result
    let task: Task<LayerStack> = thread_pool.spawn(async move {
        info!("Starting world creation");

        let layer_width: usize = 320;
        let layer_height: usize = 320;

        let mut layer_stack = LayerStack::new();
        layer_stack.add_layers(
            GroundLayerBuilder::new(layer_width, layer_height, GroundTileType::Grass)
                .add_edges_and_corner_tiles_for_base_tile(
                    GroundTileType::Grass,
                    vec![
                        (
                            EdgeDirection::OutsideTopLeft,
                            GroundTileType::GrassEdgeOutsideTopLeft,
                        ),
                        (EdgeDirection::Top, GroundTileType::GrassEdgeTop),
                        (
                            EdgeDirection::OutsideTopRight,
                            GroundTileType::GrassEdgeOutsideTopRight,
                        ),
                        (EdgeDirection::Right, GroundTileType::GrassEdgeRight),
                        (
                            EdgeDirection::OutsideBottomRight,
                            GroundTileType::GrassEdgeOutsideBottomRight,
                        ),
                        (EdgeDirection::Bottom, GroundTileType::GrassEdgeBottom),
                        (
                            EdgeDirection::OutsideBottomLeft,
                            GroundTileType::GrassEdgeOutsideBottomLeft,
                        ),
                        (EdgeDirection::Left, GroundTileType::GrassEdgeLeft),
                    ],
                )
                .generate_variegated_base(GroundTileType::RockyDirt)
                // .decorate_edge_tiles_differing_from_base(
                //     GroundTileType::Grass,
                //     (
                //         GroundTileType::GrassEdgeTop,
                //         GroundTileType::GrassEdgeRight,
                //         GroundTileType::GrassEdgeBottom,
                //         GroundTileType::GrassEdgeLeft,
                //     ),
                // )
                // .replace_inside_top_left_corners(
                //     GroundTileType::Grass,
                //     GroundTileType::GrassEdgeInsideCornerTopLeft,
                // )
                // Add alternatives last to simplify processing of tiles.
                .add_alternative_for(
                    GroundTileType::Grass,
                    GroundTileType::GrassAlternate1,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::Grass,
                    GroundTileType::GrassAlternate2,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::Grass,
                    GroundTileType::GrassAlternate3,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::RockyDirt,
                    GroundTileType::RockyDirtAlternate1,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::RockyDirt,
                    GroundTileType::RockyDirtAlternate2,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::RockyDirt,
                    GroundTileType::RockyDirtAlternate3,
                    0.033,
                )
                .build(),
        );

        info!("World creation complete");
        layer_stack
    });

    commands.insert_resource(ComputeLayerTask { task });
    next_state.set(LoadingState::CreatingGameWorld);
}

// Add a system to poll for the task completion and insert the resource
fn poll_layer_stack_task(
    mut commands: Commands,
    mut compute_layer_task: ResMut<ComputeLayerTask>,
    mut next_state: ResMut<NextState<LoadingState>>,
) {
    if let Some(layer_stack) = block_on(future::poll_once(&mut compute_layer_task.task)) {
        commands.insert_resource(layer_stack);
        next_state.set(LoadingState::ReticulatingSplines);
    }
}

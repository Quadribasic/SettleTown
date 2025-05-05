use std::time::Duration;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{Cursor, InternalWindowState, PresentMode, WindowCloseRequested},
};
use plugins::{
    game_world_window::{GameWorldWindow, SettleTownGameWorldWindowPlugin},
    splash_screen::{SettleTownLoadingSplashScreenPlugin, SplashScreenSprite},
};
use states::game_states::GameState;

// use crate::plugins::game_world_window::SettleTownGameWorldWindowPlugin;

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

struct DurationSplashScreenVisible(Duration);

impl Default for DurationSplashScreenVisible {
    fn default() -> Self {
        DurationSplashScreenVisible(Duration::from_secs(5))
    }
}

fn main() {
    App::new()
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
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
            is_fullscreen: false,
            window_width: 1408,
            window_height: 768,
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

fn spawn_game_world_window(mut commands: Commands) {
    info!("Creating game window");
    commands.spawn((
        Window {
            cursor: Cursor::default(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            mode: bevy::window::WindowMode::Windowed,
            position: WindowPosition::default(),
            resolution: (1408.0, 768.0).into(),
            title: "SettleTown".into(),
            name: Some("com.quadribasic.settletown.main".into()),
            composite_alpha_mode: bevy::window::CompositeAlphaMode::Auto,
            resize_constraints: WindowResizeConstraints::default(),
            resizable: false,
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            decorations: true,
            transparent: false,
            focused: true,
            window_level: bevy::window::WindowLevel::Normal,
            canvas: None,
            fit_canvas_to_parent: false,
            prevent_default_event_handling: false,
            internal: InternalWindowState::default(),
            ime_enabled: false,
            visible: true,
            ..Default::default()
        },
        GameWorldWindow,
    ));
}

fn make_main_window_invisible(
    mut commands: Commands,
    mut window: Query<(Entity, &mut Window)>,
    splash_screen_sprite: Query<Entity, With<SplashScreenSprite>>,
    mut event_close: EventWriter<WindowCloseRequested>,
) {
    // Despawn the splash screen sprite unless we want it to stay around inside the game world,
    // which we don't.
    commands.entity(splash_screen_sprite.single()).despawn();

    // Winit does not support updating transparency after window creation, so we
    // need to destroy the original window and create a new one.
    let (window_entity, mut window) = window.single_mut();
    info!("Making splash screen window invisible");
    window.visible = false;

    info!(
        "Despawning splash screen window (Entity: {:?})",
        window_entity
    );
    commands.entity(window_entity).despawn();

    spawn_game_world_window(commands);

    // info!("Dispatching event to close splash screen window");
    // event_close.send(WindowCloseRequested {
    //     window: window_entity,
    // });
}

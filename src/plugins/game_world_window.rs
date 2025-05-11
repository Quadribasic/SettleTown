use bevy::{app::Plugin, prelude::*};
use bevy_state::state::{OnTransition, States};

use super::{
    animation::SettleTownSpriteAnimatorPlugin, camera::SettleTownCameraPlugin,
    input::SettleTownInputPlugin, layer::SettleTownLayerPlugin,
    player_character::SettleTownPlayerCharacterPlugin,
};
use crate::states::game_states::GameState;

/// The default width of the game world window, in device pixels.
pub const DEFAULT_WINDOW_WIDTH: f32 = 1408.0;

// The default height of the game world window, in device pixels.
pub const DEFAULT_WINDOW_HEIGHT: f32 = 768.0;

/// The default tile width for layers, in device pixels.
const DEFAULT_LAYER_TILE_WIDTH: usize = 320;

/// The default tile height for layers, in device pixels.
const DEFAULT_LAYER_TILE_HEIGHT: usize = 320;

/// An event that is emitted when the game world window is created.
#[derive(Event)]
pub struct GameWorldWindowCreated {
    pub window_entity: Entity,
}

/// A marker component for the game world window.
#[derive(Component)]
pub struct GameWorldWindow;

pub struct SettleTownGameWorldWindowPlugin<S: States> {
    pub enabled_state: S,
    pub _is_fullscreen: bool,
    pub window_width: i32,
    pub window_height: i32,
}

impl<S: States> Plugin for SettleTownGameWorldWindowPlugin<S> {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<GameWorldWindowCreated>().add_systems(
            OnEnter(self.enabled_state.clone()),
            create_game_world_window,
        );

        app.add_plugins(SettleTownCameraPlugin {
            width: self.window_width as f32,
            height: self.window_height as f32,
            enabled_state: self.enabled_state.clone(),
        });
        app.add_plugins(SettleTownLayerPlugin::new(
            DEFAULT_LAYER_TILE_WIDTH,
            DEFAULT_LAYER_TILE_HEIGHT,
            self.enabled_state.clone(),
        ));
        app.add_plugins(SettleTownPlayerCharacterPlugin {
            enabled_state: GameState::InGameWorld,
        });
        app.add_plugins(SettleTownInputPlugin {
            enabled_state: GameState::InGameWorld,
        });
        app.add_plugins(SettleTownSpriteAnimatorPlugin {
            _enabled_state: GameState::InGameWorld,
        });
        // The world inspector plugin allows us to take a look at the world
        // so we can see bounding boxes, etc.
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_systems(
        //     Update,
        //     (
        //         // change_title,
        //         // toggle_theme,
        //         // toggle_cursor,
        //         // toggle_vsync,
        //         // toggle_window_controls,
        //         // cycle_cursor_icon,
        //         // switch_level,
        //         make_game_world_window_visible.run_if(in_state(self.enabled_state.clone())),
        //         // random,
        //         calculate_bounds_2d,
        //         // toggle_aabb_rendering,
        //     ),
        // );
        // .insert_resource(SettleTownConfig {
        //     show_bounding_boxes: false,
        // }))
    }
}

fn create_game_world_window(
    mut commands: Commands,
    mut message_bus: EventWriter<GameWorldWindowCreated>,
) {
    // Create the game world window
    info!("Creating game world window");
    let window_entity = commands
        .spawn((
            Window {
                title: "SettleTown".into(),
                resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            },
            GameWorldWindow,
        ))
        .id();

    message_bus.send(GameWorldWindowCreated { window_entity });
}

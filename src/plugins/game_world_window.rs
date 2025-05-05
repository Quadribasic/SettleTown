use bevy::{
    app::Plugin,
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::calculate_bounds_2d,
    utils::default,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

use super::{
    animation::SettleTownSpriteAnimatorPlugin, camera::SettleTownCameraPlugin,
    input::SettleTownInputPlugin, layer::SettleTownLayerPlugin,
    player_character::SettleTownPlayerCharacterPlugin,
};
use crate::states::game_states::GameState;

/// The default width of the game world window, in device pixels.
const DEFAULT_WINDOW_WIDTH: i32 = 1408;

// The default height of the game world window, in device pixels.
const DEFAULT_WINDOW_HEIGHT: i32 = 768;

/// The default tile width for layers, in device pixels.
const DEFAULT_LAYER_TILE_WIDTH: usize = 320;

/// The default tile height for layers, in device pixels.
const DEFAULT_LAYER_TILE_HEIGHT: usize = 320;

/// A marker component for the game world window.
#[derive(Component)]
pub struct GameWorldWindow;

pub struct SettleTownGameWorldWindowPlugin<S: States> {
    pub enabled_state: S,
    pub is_fullscreen: bool,
    pub window_width: i32,
    pub window_height: i32,
}

// impl SettleTownGameWorldWindowPlugin {
// fn make_game_world_window_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
//     // The delay may be different for your app or system.
//     if frames.0 == 3 {
//         // At this point the gpu is ready to show the app so we can make the window visible.
//         // Alternatively, you could toggle the visibility in Startup.
//         // It will work, but it will have one white frame before it starts rendering
//         window.single_mut().visible = true;
//     }
// }
// }

// impl<S: States> Default for SettleTownGameWorldWindowPlugin<S> {
//     fn default() -> Self {
//         Self {
//             is_fullscreen: Default::default(),
//             window_width: DEFAULT_WINDOW_WIDTH,
//             window_height: DEFAULT_WINDOW_HEIGHT,
//         }
//     }
// }

impl<S: States> Plugin for SettleTownGameWorldWindowPlugin<S> {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(SettleTownCameraPlugin {
            width: self.window_width as f32,
            height: self.window_height as f32,
            enabled_state: self.enabled_state.clone(),
        })
        .add_plugins(SettleTownLayerPlugin::new(
            DEFAULT_LAYER_TILE_WIDTH,
            DEFAULT_LAYER_TILE_HEIGHT,
            self.enabled_state.clone(),
        ))
        .add_plugins(SettleTownPlayerCharacterPlugin {
            enabled_state: GameState::InGameWorld,
        })
        .add_plugins(SettleTownInputPlugin {
            enabled_state: GameState::InGameWorld,
        })
        .add_plugins(SettleTownSpriteAnimatorPlugin {
            enabled_state: GameState::InGameWorld,
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

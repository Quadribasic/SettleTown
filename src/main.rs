use std::time::Duration;

use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod components;
mod plugins;
mod resources;

// use bevy_tiles::{commands::TileCommandExt, coords::CoordIterator, tiles_2d::*, TilesPlugin};
use plugins::{input::SettleTownInputPlugin, map::SettleTownMapPlugin};

// use components::world_position::WorldPosition;
// use resources::map::Map;

use resources::ground::Map;

const WINDOW_WIDTH: i32 = 1408;
const WINDOW_HEIGHT: i32 = 768;

fn main() {
    // let args = Args::parse();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SettleTown".into(),
                        // name: Some("com.quadribasic.settletown.main".into()),
                        resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                        // present_mode: PresentMode::AutoVsync,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        // window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        // This will spawn an invisible window
                        // The window will be made visible in the make_visible() system after 3 frames.
                        // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(SettleTownInputPlugin)
        .add_plugins(SettleTownMapPlugin)
        .insert_resource(Map::new(320, 320))
        .add_systems(
            Update,
            (
                // change_title,
                // toggle_theme,
                // toggle_cursor,
                // toggle_vsync,
                // toggle_window_controls,
                // cycle_cursor_icon,
                // switch_level,
                make_window_visible,
                // random,
                // execute_animations,
            ),
        )
        .run();
}

fn make_window_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}

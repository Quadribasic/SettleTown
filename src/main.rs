use bevy::prelude::*;

use crate::plugins::game_world_window::SettleTownGameWorldWindowPlugin;

mod components;
mod plugins;
mod resources;
mod systems;

// use bevy_tiles::{commands::TileCommandExt, coords::CoordIterator, tiles_2d::*, TilesPlugin};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use plugins::{
//     animation::SettleTownSpriteAnimatorPlugin, camera::SettleTownCameraPlugin,
//     input::SettleTownInputPlugin, layer::SettleTownLayerPlugin,
//     player_character::SettleTownPlayerCharacterPlugin,
// };

fn main() {
    // App::new()
    //     .add_plugins(SettleTownLoadingSplashScreenPlugin)
    //     .run();
    App::new()
        .add_plugins(SettleTownGameWorldWindowPlugin {
            ..Default::default()
        })
        .run();
}

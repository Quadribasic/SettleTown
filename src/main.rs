use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_just_pressed,
    prelude::*,
    sprite::calculate_bounds_2d,
};

mod components;
mod plugins;
mod resources;
mod systems;

// use bevy_tiles::{commands::TileCommandExt, coords::CoordIterator, tiles_2d::*, TilesPlugin};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::{
    camera::SettleTownCameraPlugin,
    input::SettleTownInputPlugin,
    layer::SettleTownLayerPlugin,
    player_character::{AnimationConfig, PlayerCharacter, SettleTownPlayerCharacterPlugin},
};

use crate::components::sprites::Direction;

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
                // Setting the ImagePlugin to default_nearest makes the pixels sharp (which we want
                // for a pixel art game). It defaults to linear, which is an interpolation and will
                // make things look blurry.
                .set(ImagePlugin::default_nearest()),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(SettleTownCameraPlugin::new(
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32,
        ))
        .add_plugins(SettleTownLayerPlugin::new(320, 320))
        .add_plugins(SettleTownPlayerCharacterPlugin)
        .add_plugins(SettleTownInputPlugin)
        // The world inspector plugin allows us to take a look at the world
        // so we can see bounding boxes, etc.
        // .add_plugins(WorldInspectorPlugin::new())
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
                calculate_bounds_2d,
                // toggle_aabb_rendering,
            ),
        )
        .add_systems(
            Update,
            (
                trigger_animation::<PlayerCharacter>
                    .run_if(input_just_pressed(KeyCode::ArrowRight)),
                execute_animations_indefinite,
            )
                .chain(),
        )
        // .insert_resource(SettleTownConfig {
        //     show_bounding_boxes: false,
        // })
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

pub fn execute_animations_indefinite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas), With<PlayerCharacter>>,
) {
    // This loops through all the frames in the texture atlas, from first to last frame
    // as defined in the AnimationConfig.
    for (mut config, mut atlas) in &mut query {
        if config.first_sprite_index == 18 {
            return;
        }

        if config.should_run {
            // info!(
            //     "Running animation with first sprite index: {:?}",
            //     config.first_sprite_index
            // );
            // we track how long the current sprite has been displayed for
            config.frame_timer.tick(time.delta());

            // If it has been displayed for the user-defined amount of time (fps)...
            if config.frame_timer.just_finished() {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame
                    atlas.index = config.first_sprite_index;

                    // ... and reset the frame timer.
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                } else {
                    info!(
                        "Updating frame from {:?} to {:?}",
                        atlas.index,
                        atlas.index + 1
                    );
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

fn trigger_animation<S: Component>(
    mut query: Query<(&mut AnimationConfig, &mut Direction, &mut TextureAtlas), With<S>>,
) {
    // we expect the Component of type S to be used as a marker Component by only a single entity
    let mut animation = query.single_mut();
    info!("Triggering animation with direction: {:?}", animation.1);
    // we create a new timer when the animation is triggered
    animation.0.first_sprite_index = (animation.1.clone() as usize) * 9;
    animation.0.last_sprite_index = animation.0.first_sprite_index + 8;
    animation.2.index = animation.0.first_sprite_index;

    animation.0.frame_timer = AnimationConfig::timer_from_fps(animation.0.fps);
    animation.0.should_run = true;
}

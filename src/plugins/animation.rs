use std::time::Duration;

use bevy::{
    app::{Plugin, Update},
    input::common_conditions::input_just_pressed,
    log::info,
    prelude::{Component, IntoSystemConfigs, KeyCode, Query, Res, With},
    sprite::TextureAtlas,
    time::{Time, Timer, TimerMode},
};

use crate::components::sprites::{Direction, DirectionalSprite};

// XXX_jwir3: We should probably externalize this, as it's not going to be limited to _only_ the
//            player character, but rather all sprites.
#[derive(Component, Debug)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
    pub should_run: bool,
}

impl AnimationConfig {
    pub fn new(first_frame: usize, last_frame: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first_frame,
            last_sprite_index: last_frame,
            fps: fps,
            frame_timer: Self::timer_from_fps(fps),
            should_run: false,
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub struct SettleTownSpriteAnimatorPlugin;

impl Plugin for SettleTownSpriteAnimatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                trigger_animation::<DirectionalSprite>
                    .run_if(input_just_pressed(KeyCode::ArrowRight)),
                trigger_animation::<DirectionalSprite>
                    .run_if(input_just_pressed(KeyCode::ArrowLeft)),
                trigger_animation::<DirectionalSprite>.run_if(input_just_pressed(KeyCode::ArrowUp)),
                trigger_animation::<DirectionalSprite>
                    .run_if(input_just_pressed(KeyCode::ArrowDown)),
                execute_animations_indefinite,
            )
                .chain(),
        );
        // app.add_systems(Startup, spawn_player_character_entity);
    }
}

pub fn execute_animations_indefinite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas), With<DirectionalSprite>>,
) {
    // This loops through all the frames in the texture atlas, from first to last frame
    // as defined in the AnimationConfig.
    for (mut config, mut atlas) in &mut query {
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
    info!(
        "Setting first sprite index to: {:?}",
        animation.0.first_sprite_index
    );
    animation.0.last_sprite_index = animation.0.first_sprite_index + 8;
    info!(
        "Setting last sprite index to: {:?}",
        animation.0.last_sprite_index
    );
    animation.2.index = animation.0.first_sprite_index;

    animation.0.frame_timer = AnimationConfig::timer_from_fps(animation.0.fps);
    animation.0.should_run = true;
}

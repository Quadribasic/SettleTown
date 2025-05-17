use std::time::Duration;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::{Condition, IntoScheduleConfigs},
        system::Single,
    },
    input::common_conditions::input_just_pressed,
    log::info,
    prelude::{Component, KeyCode, Query, Res, Sprite, State, TextureAtlas, With},
    time::{Time, Timer, TimerMode},
};
use bevy_state::state::States;

use crate::{
    components::sprites::{Direction, DirectionalSprite},
    states::game_states::GameState,
};

// XXX_jwir3: We should probably externalize this, as it's not going to be limited to _only_ the
//            player character, but rather all sprites.
#[derive(Component, Debug)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
    pub should_run_animation: bool,
}

impl AnimationConfig {
    pub fn new(first_frame: usize, last_frame: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first_frame,
            last_sprite_index: last_frame,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            should_run_animation: false,
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub struct SettleTownSpriteAnimatorPlugin<S: States> {
    pub _enabled_state: S,
}

impl<S: States> Plugin for SettleTownSpriteAnimatorPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                trigger_animation::<DirectionalSprite>.run_if(
                    input_just_pressed(KeyCode::ArrowRight)
                        .or(input_just_pressed(KeyCode::ArrowLeft))
                        .or(input_just_pressed(KeyCode::ArrowUp))
                        .or(input_just_pressed(KeyCode::ArrowDown))
                        .or(input_just_pressed(KeyCode::KeyW))
                        .or(input_just_pressed(KeyCode::KeyA))
                        .or(input_just_pressed(KeyCode::KeyS))
                        .or(input_just_pressed(KeyCode::KeyD)),
                ),
                // execute_animations_indefinite.run_if(in_state(self.enabled_state.clone())),
                execute_animations_indefinite,
            )
                .chain(),
        );
        // app.add_systems(Startup, spawn_player_character_entity);
    }
}

pub fn execute_animations_indefinite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<DirectionalSprite>>,
    current_state: Res<State<GameState>>,
) {
    // XXX_jwir3: This is a bit of a hack, but if we don't do this and instead use .run_if
    // for this system, then it will cause issues with the animation not picking up the
    // correct initial frame.
    if *current_state.get() != GameState::InGameWorld {
        return;
    }

    // This loops through all the frames in the texture atlas, from first to last frame
    // as defined in the AnimationConfig.
    for (mut config, mut sprite) in &mut query {
        if config.should_run_animation {
            // info!(
            //     "Running animation for frames: {:?} through {:?}",
            //     config.first_sprite_index, config.last_sprite_index,
            // );

            // we track how long the current sprite has been displayed for
            config.frame_timer.tick(time.delta());

            // If it has been displayed for the user-defined amount of time (fps)...
            // if config.frame_timer.just_finished() {
            //     if let Some(atlas) = &mut sprite.texture_atlas {
            //         if atlas.index == config.last_sprite_index {
            //             // ...and it IS the last frame, then we move back to the first frame and stop.
            //             atlas.index = config.first_sprite_index;
            //         } else {
            //             // ...and it is NOT the last frame, then we move to the next frame...
            //             atlas.index += 1;
            //             // ...and reset the frame timer to start counting all over again
            //             config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            //         }
            //     }
            // }

            // If it has been displayed for the user-defined amount of time (fps)...
            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
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
        } else if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = config.first_sprite_index;
        }
    }
}

fn trigger_animation<S: Component>(
    mut animation: Single<(&mut AnimationConfig, &mut Direction, &mut Sprite), With<S>>,
    current_state: Res<State<GameState>>,
) {
    if *current_state.get() != GameState::InGameWorld {
        return;
    }

    info!("Triggering animation with direction: {:?}", animation.1);

    // Calculate the base index for the current direction's animation frames
    let direction_index = (*animation.1).clone() as usize;
    animation.0.first_sprite_index = direction_index * 9;
    animation.0.last_sprite_index = animation.0.first_sprite_index + 8;

    // Set the current frame to the first frame of the animation
    if let Some(texture_atlas) = animation.2.texture_atlas.clone() {
        animation.2.texture_atlas = Some(TextureAtlas {
            index: animation.0.first_sprite_index,
            layout: texture_atlas.layout,
        });
    }

    animation.0.frame_timer = AnimationConfig::timer_from_fps(animation.0.fps);
    animation.0.should_run_animation = true;
}

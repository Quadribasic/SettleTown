use bevy::prelude::*;

use super::{camera::OverheadMapCamera, player_character::PlayerCharacter};
use crate::components::sprites::Direction;

const CAMERA_MOVEMENT_SPEED: f32 = 0.01;
const PLAYER_MOVEMENT_SPEED: f32 = 1.0;

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_camera: Query<&mut OrthographicProjection, With<OverheadMapCamera>>,
    mut query_player_character: Query<
        (&mut Direction, &mut Transform, &mut TextureAtlas),
        With<PlayerCharacter>,
    >,
) {
    if keyboard_input.pressed(KeyCode::KeyQ) {
        info!("User requested quitting the application from the keyboard");
        std::process::exit(0);
    }

    let mut projection = query_camera.single_mut();

    let mut player_char = query_player_character.single_mut();
    // Update the player character's facing direction and the position of the
    // player character.
    let mut pc_movement: Vec3 = player_char.1.translation.clone();
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        *player_char.0 = Direction::East;
        pc_movement = Vec3::new(
            pc_movement.x + PLAYER_MOVEMENT_SPEED,
            pc_movement.y,
            pc_movement.z,
        );
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        *player_char.0 = Direction::West;
        pc_movement = Vec3::new(
            pc_movement.x - PLAYER_MOVEMENT_SPEED,
            pc_movement.y,
            pc_movement.z,
        );
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        *player_char.0 = Direction::North;
        pc_movement = Vec3::new(
            pc_movement.x,
            pc_movement.y + PLAYER_MOVEMENT_SPEED,
            pc_movement.z,
        );
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        *player_char.0 = Direction::South;
        pc_movement = Vec3::new(
            pc_movement.x,
            pc_movement.y - PLAYER_MOVEMENT_SPEED,
            pc_movement.z,
        );
    }

    // Also update the texture atlas index
    player_char.2.index = (player_char.0.clone() as usize) * 9;

    let mut horizontal_movement = 0.0;
    let mut vertical_movement = 0.0;

    let mut shift_multiplier = 1.0;

    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        shift_multiplier = 10.0;
    }

    // if keyboard_input.pressed(KeyCode::ArrowRight) {
    //     horizontal_movement -= shift_multiplier * CAMERA_MOVEMENT_SPEED;
    // } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
    //     horizontal_movement += shift_multiplier * CAMERA_MOVEMENT_SPEED;
    // }

    // if keyboard_input.pressed(KeyCode::ArrowUp) {
    //     vertical_movement -= shift_multiplier * CAMERA_MOVEMENT_SPEED;
    // } else if keyboard_input.pressed(KeyCode::ArrowDown) {
    //     vertical_movement += shift_multiplier * CAMERA_MOVEMENT_SPEED;
    // }

    projection.viewport_origin = Vec2::new(
        projection.viewport_origin.x + horizontal_movement,
        projection.viewport_origin.y + vertical_movement,
    );

    player_char.1.translation = pc_movement;
}

pub struct SettleTownInputPlugin;

impl Plugin for SettleTownInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input_system);
        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        // app.add_systems(Startup, add_people);
        // app.add_systems(Update, (update_people, greet_people).chain());
    }
}

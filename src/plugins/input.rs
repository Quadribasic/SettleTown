use bevy::prelude::*;

use super::{
    camera::{OverheadMapCamera, ViewportWorldSize},
    player_character::PlayerCharacter,
};
use crate::{components::sprites::Direction, resources::world::WorldCoordinates};

const PLAYER_MOVEMENT_SPEED: f32 = 1.0;

fn keyboard_input_system(
    // commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_camera: Query<
        (&mut Camera, &mut Transform, &GlobalTransform),
        (With<OverheadMapCamera>, Without<PlayerCharacter>),
    >,
    mut query_player_character: Query<
        (&mut Direction, &mut Transform, &mut TextureAtlas),
        With<PlayerCharacter>,
    >,
    world_coords: Res<WorldCoordinates>,
    viewport_size: Res<ViewportWorldSize>,
) {
    if keyboard_input.pressed(KeyCode::KeyQ) {
        info!("User requested quitting the application from the keyboard");
        std::process::exit(0);
    }

    let mut horizontal_camera_movement = 0.0;
    let mut vertical_camera_movement = 0.0;

    let mut shift_multiplier = 1.0;

    // If we have the SHIFT key pressed, then we'll increase the rate of transition rate.
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        shift_multiplier = 10.0;
    }

    let mut player_char = query_player_character.single_mut();
    let player_char_original_pos = player_char.1.translation.clone();

    // Update the player character's facing direction and the position of the
    // player character.
    let mut pc_movement: Vec3 = player_char.1.translation.clone();
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        *player_char.0 = Direction::East;
        pc_movement = Vec3::new(
            pc_movement.x + shift_multiplier * PLAYER_MOVEMENT_SPEED,
            pc_movement.y,
            pc_movement.z,
        );
        horizontal_camera_movement += shift_multiplier * PLAYER_MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        *player_char.0 = Direction::West;
        pc_movement = Vec3::new(
            pc_movement.x - shift_multiplier * PLAYER_MOVEMENT_SPEED,
            pc_movement.y,
            pc_movement.z,
        );
        horizontal_camera_movement -= shift_multiplier * PLAYER_MOVEMENT_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        *player_char.0 = Direction::North;
        pc_movement = Vec3::new(
            pc_movement.x,
            pc_movement.y + shift_multiplier * PLAYER_MOVEMENT_SPEED,
            pc_movement.z,
        );
        vertical_camera_movement += shift_multiplier * PLAYER_MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        *player_char.0 = Direction::South;
        pc_movement = Vec3::new(
            pc_movement.x,
            pc_movement.y - shift_multiplier * PLAYER_MOVEMENT_SPEED,
            pc_movement.z,
        );

        vertical_camera_movement -= shift_multiplier * PLAYER_MOVEMENT_SPEED;
    }

    pc_movement = Vec3::new(
        f32::max(
            f32::min(pc_movement.x, world_coords.get_max_x()),
            world_coords.get_min_x(),
        ),
        f32::max(
            f32::min(pc_movement.y, world_coords.get_max_y()),
            world_coords.get_min_y(),
        ),
        pc_movement.z,
    );

    // Set the sprite's translation
    player_char.1.translation = pc_movement;

    // info!(
    //     "Player is now at: ({:?}, {:?})",
    //     player_char.1.translation.x, player_char.1.translation.y
    // );

    // Also update the texture atlas index
    player_char.2.index = (player_char.0.clone() as usize) * 9;

    // Update the camera

    // Only move the camera until the left edge aligns with the left edge of the world, the right
    // edge aligns with the right edge of the world, the top edge aligns with the top edge of
    // the world, or the bottom edge aligns with the bottom edge of the world.
    let mut camera = query_camera.single_mut();

    let viewport_half_width = viewport_size.width / 2.0;
    let viewport_half_height = viewport_size.height / 2.0;

    // If the player is moving _towards_ the center of the viewport, then we don't want to
    // move the camera.
    let viewport_center_world = camera
        .0
        .ndc_to_world(camera.2, Vec3::new(0.0, 0.0, 0.0))
        .unwrap();

    let distance_to_center_before_move = (player_char_original_pos - viewport_center_world).abs();
    let distance_to_center_after_move = (player_char.1.translation - viewport_center_world).abs();

    if distance_to_center_before_move.x > distance_to_center_after_move.x {
        horizontal_camera_movement = 0.0;
    }

    if distance_to_center_before_move.y > distance_to_center_after_move.y {
        vertical_camera_movement = 0.0;
    }

    info!(
        "Distance to camera center before move: {:?}",
        distance_to_center_before_move
    );
    info!(
        "Distance to camera center after move: {:?}",
        distance_to_center_after_move
    );
    info!(
        "Viewport center (in world coords): ({:?}, {:?})",
        viewport_center_world.x, viewport_center_world.y
    );

    camera.1.translation = Vec3::new(
        f32::max(
            f32::min(
                camera.1.translation.x + horizontal_camera_movement,
                world_coords.get_max_x() - viewport_half_width,
            ),
            world_coords.get_min_x() + viewport_half_width,
        ),
        f32::max(
            f32::min(
                camera.1.translation.y + vertical_camera_movement,
                world_coords.get_max_y() - viewport_half_height,
            ),
            world_coords.get_min_y() + viewport_half_height,
        ),
        camera.1.translation.z,
    );
}

pub struct SettleTownInputPlugin;

impl Plugin for SettleTownInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input_system);
    }
}

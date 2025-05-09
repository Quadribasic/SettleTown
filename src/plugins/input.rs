use bevy::prelude::*;

use super::{
    animation::{execute_animations_indefinite, AnimationConfig},
    camera::{OverheadMapCamera, ViewportWorldSize},
    player_character::{PlayerCharacter, PLAYER_CHARACTER_RUN_FPS, PLAYER_CHARACTER_WALK_FPS},
};
use crate::{components::sprites::Direction, resources::world::WorldCoordinates};

const PLAYER_MOVEMENT_SPEED: f32 = 1.0;

// A query for assets that apparel pieces for a character. These have a direction and a
// texture atlas from which the sprites are retrieved. They are children of the character asset.
// type ApparelQuery<'a> = (&'a Parent, &'a mut Direction, &'a mut TextureAtlas);

/// A query for assets that are cameras in the scene.
type CameraQuery<'a> = (&'a mut Camera, &'a mut Transform, &'a GlobalTransform);

/// A query for assets that are players (a specific type of character) in the scene.
type PlayerQuery<'a> = (
    &'a mut Direction,
    &'a mut Transform,
    &'a mut TextureAtlas,
    &'a mut AnimationConfig,
);

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_camera: Query<CameraQuery, (With<OverheadMapCamera>, Without<PlayerCharacter>)>,
    mut query_player_character: Query<PlayerQuery, With<PlayerCharacter>>,
    world_coords: Res<WorldCoordinates>,
    viewport_size: Res<ViewportWorldSize>,
) {
    if keyboard_input.pressed(KeyCode::KeyQ) {
        std::process::exit(0);
    }

    let mut horizontal_camera_movement = 0.0;
    let mut vertical_camera_movement = 0.0;

    let mut shift_multiplier = 1.0;

    let mut player_char = query_player_character.single_mut();

    // If we have the SHIFT key pressed, then we'll increase the rate of transition rate.
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        shift_multiplier = 10.0;
        player_char.3.fps = PLAYER_CHARACTER_RUN_FPS;
    } else {
        player_char.3.fps = PLAYER_CHARACTER_WALK_FPS;
    }

    let player_char_original_pos = player_char.1.translation;

    // Update the player character's facing direction and the position of the
    // player character.
    let mut pc_movement: Vec3 = player_char.1.translation;
    if [KeyCode::ArrowRight, KeyCode::KeyD]
        .iter()
        .any(|&key| keyboard_input.pressed(key))
    {
        // info!("Setting direction to EAST");
        *player_char.0 = Direction::East;
        pc_movement = Vec3::new(
            pc_movement.x + shift_multiplier * PLAYER_MOVEMENT_SPEED,
            pc_movement.y,
            pc_movement.z,
        );
        horizontal_camera_movement += shift_multiplier * PLAYER_MOVEMENT_SPEED;
    } else if [KeyCode::ArrowLeft, KeyCode::KeyA]
        .iter()
        .any(|&key| keyboard_input.pressed(key))
    {
        *player_char.0 = Direction::West;
        // info!("Setting direction to WEST");
        pc_movement = Vec3::new(
            pc_movement.x - shift_multiplier * PLAYER_MOVEMENT_SPEED,
            pc_movement.y,
            pc_movement.z,
        );
        horizontal_camera_movement -= shift_multiplier * PLAYER_MOVEMENT_SPEED;
    }

    if [KeyCode::ArrowUp, KeyCode::KeyW]
        .iter()
        .any(|&key| keyboard_input.pressed(key))
    {
        *player_char.0 = Direction::North;
        // info!("Setting direction to NORTH");
        pc_movement = Vec3::new(
            pc_movement.x,
            pc_movement.y + shift_multiplier * PLAYER_MOVEMENT_SPEED,
            pc_movement.z,
        );
        vertical_camera_movement += shift_multiplier * PLAYER_MOVEMENT_SPEED;
    } else if [KeyCode::ArrowDown, KeyCode::KeyS]
        .iter()
        .any(|&key| keyboard_input.pressed(key))
    {
        *player_char.0 = Direction::South;
        // info!("Setting direction to SOUTH");
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

    if [
        KeyCode::ArrowRight,
        KeyCode::ArrowLeft,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::KeyW,
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
    ]
    .iter()
    .any(|key| keyboard_input.just_released(*key))
    {
        // Stop the player character's walking animation and reset it to the appropriate frame.
        player_char.3.should_run_animation = false;
    }

    // Also update the animation for the direction the player is facing
    player_char.3.first_sprite_index = (player_char.0.clone() as usize) * 9;
    player_char.3.last_sprite_index = player_char.3.first_sprite_index + 8;

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

    // info!(
    //     "Distance to camera center before move: {:?}",
    //     distance_to_center_before_move
    // );
    // info!(
    //     "Distance to camera center after move: {:?}",
    //     distance_to_center_after_move
    // );
    // info!(
    //     "Viewport center (in world coords): ({:?}, {:?})",
    //     viewport_center_world.x, viewport_center_world.y
    // );

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

pub struct SettleTownInputPlugin<S: States> {
    pub enabled_state: S,
}

impl<S: States> Plugin for SettleTownInputPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            keyboard_input_system
                .before(execute_animations_indefinite)
                .run_if(in_state(self.enabled_state.clone())),
        );

        // app.add_systems(Update, update_child_directions.after(keyboard_input_system));
    }
}

// fn update_child_directions(
//     parent_query: Query<&Direction, With<PlayerCharacter>>,
//     mut child_query: Query<ApparelQuery, (With<Apparel>, Without<PlayerCharacter>)>,
// ) {
//     // XXX_jwir3: We'll need to make sure that we iterate through _all_ apparel once we have
//     // multiple possible pieces.
//     let player_char_direction = parent_query.single();
//     let child = child_query.get_single_mut();
//     if let Ok(mut child_val) = child {
//         *child_val.1 = player_char_direction.clone();
//         child_val.2.index = player_char_direction.clone() as usize;
//     }
//     // for mut child in child_query.iter_mut() {
//     //     child.1 = player_char_direction.clone();
//     //     child.2 = TextureAtlas {
//     //         layout: child.2.layout,
//     //         index: (*player_char_direction) as usize,
//     //     };
//     //     // child.2.index = (*player_char_direction) as usize;
//     //     info!("Apparel facing direction is now: {:?}", child.1);
//     // }

//     // // get the properties of each squad
//     // for (squad_dmg, children) in q_parent.iter() {
//     //     // `children` is a collection of Entity IDs
//     //     for &child in children.iter() {
//     //         // get the health of each child unit
//     //         let health = q_child.get(child);

//     //         // do something
//     //     }
//     // }
// }

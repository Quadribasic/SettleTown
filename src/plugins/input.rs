use bevy::prelude::*;

use super::map::OverheadMapCamera;

const CAMERA_MOVEMENT_SPEED: f32 = 0.01;

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_camera: Query<&mut OrthographicProjection, With<OverheadMapCamera>>,
) {
    if keyboard_input.pressed(KeyCode::KeyQ) {
        info!("User requested quitting the application from the keyboard");
        std::process::exit(0);
    }

    let mut projection = query_camera.single_mut();

    let mut horizontal_movement = 0.0;
    let mut vertical_movement = 0.0;

    let mut shift_multiplier = 1.0;

    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        shift_multiplier = 10.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        horizontal_movement -= shift_multiplier * CAMERA_MOVEMENT_SPEED
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        horizontal_movement += shift_multiplier * CAMERA_MOVEMENT_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        vertical_movement -= shift_multiplier * CAMERA_MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        vertical_movement += shift_multiplier * CAMERA_MOVEMENT_SPEED;
    }

    projection.viewport_origin = Vec2::new(
        projection.viewport_origin.x + horizontal_movement,
        projection.viewport_origin.y + vertical_movement,
    );

    // info!(
    //     "Viewport Origin is now: ({}, {})",
    //     projection.viewport_origin.x, projection.viewport_origin.y
    // );
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

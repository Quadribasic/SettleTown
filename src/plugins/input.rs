use bevy::{input::keyboard, prelude::*};

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

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        horizontal_movement -= CAMERA_MOVEMENT_SPEED
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        horizontal_movement += CAMERA_MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        vertical_movement -= CAMERA_MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        vertical_movement += CAMERA_MOVEMENT_SPEED;
    }

    projection.viewport_origin = Vec2::new(
        projection.viewport_origin.x + horizontal_movement,
        projection.viewport_origin.y + vertical_movement,
    );
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

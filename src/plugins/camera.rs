use bevy::{
    app::{Plugin, Startup},
    prelude::*,
    render::camera::RenderTarget,
    window::WindowRef,
};
use bevy_state::state::States;

use super::game_world_window::GameWorldWindow;
use crate::states::game_states::GameState;

#[derive(Resource)]
pub struct ViewportWorldSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct OverheadMapCamera;

pub struct SettleTownCameraPlugin<S: States> {
    pub width: f32,
    pub height: f32,
    pub enabled_state: S,
}

impl<S: States> Plugin for SettleTownCameraPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        info!("Initializing camera plugin");
        let camera_world_size = ViewportWorldSize {
            width: self.width,
            height: self.height,
        };

        info!(
            "Camera world size: ({:?}, {:?})",
            camera_world_size.width, camera_world_size.height
        );
        app.insert_resource(camera_world_size);

        info!(
            "Enabling the setup of the camera but only running if in the state: {:?}",
            self.enabled_state
        );
        app.add_systems(OnEnter(self.enabled_state.clone()), setup_camera);
    }
}

// impl<S: States> SettleTownCameraPlugin<S> {
//     pub fn new(enabled_state: GameState, window_width_px: f32, window_height_px: f32) -> Self {
//         Self {
//             enabled_state: enabled_state.into(),
//             width: window_width_px,
//             height: window_height_px,
//         }
//     }
// }

pub fn setup_camera(mut commands: Commands, window_query: Query<Entity, With<GameWorldWindow>>) {
    info!("Setting up camera for game world");
    let camera_bundle = Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Window(WindowRef::Entity(window_query.single())),
            ..default()
        },
        ..default()
    };

    // let camera = camera_bundle.clone().camera;
    // let global_transform = camera_bundle.global_transform;
    // // The world coordinates are in pixels (1unit = 1px), but this might not always be the case.
    // // Thus, we get the height and width in world coordinates of the viewport.
    // let viewport_min_loc = camera
    //     .ndc_to_world(&global_transform, Vec3::new(-1.0, -1.0, 0.0))
    //     .unwrap();
    // let viewport_center_loc = camera
    //     .ndc_to_world(&global_transform, Vec3::new(0.0, 0.0, 0.0))
    //     .unwrap();

    commands.spawn((camera_bundle, OverheadMapCamera));
}

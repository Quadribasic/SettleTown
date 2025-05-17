use bevy::{app::Plugin, prelude::*, render::camera::RenderTarget, window::WindowRef};
use bevy_state::state::States;

use super::game_world_window::GameWorldWindowCreated;

/// An event that is emitted when the camera is setup.
#[derive(Event)]
pub struct CameraSetupEvent;

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
        app.add_event::<CameraSetupEvent>()
            .add_systems(Update, setup_camera);
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

pub fn setup_camera(
    mut commands: Commands,
    mut window_created_event: EventReader<GameWorldWindowCreated>,
    mut camera_setup_event: EventWriter<CameraSetupEvent>,
) {
    for event in window_created_event.read() {
        info!("Setting up camera for game world");

        let camera_bundle = (
            Camera2d,
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(event.window_entity)),
                ..default()
            },
        );

        commands.spawn((camera_bundle, OverheadMapCamera));

        camera_setup_event.send(CameraSetupEvent);
    }
}

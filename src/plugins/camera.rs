use bevy::{
    app::{Plugin, Startup},
    prelude::{Camera2dBundle, Commands, Component, Resource},
};

#[derive(Resource)]
pub struct ViewportWorldSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct OverheadMapCamera;

pub struct SettleTownCameraPlugin {
    width: f32,
    height: f32,
}

impl Plugin for SettleTownCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let camera_world_size = ViewportWorldSize {
            width: self.width,
            height: self.height,
        };

        app.insert_resource(camera_world_size);
        app.add_systems(Startup, setup_camera);
    }
}

impl SettleTownCameraPlugin {
    pub fn new(window_width_px: f32, window_height_px: f32) -> Self {
        Self {
            width: window_width_px,
            height: window_height_px,
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    let camera_bundle = Camera2dBundle::default();

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

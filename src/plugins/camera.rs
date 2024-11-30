use bevy::{
    app::{Plugin, Startup},
    prelude::{Camera2dBundle, Commands, Component},
};

#[derive(Component)]
pub struct OverheadMapCamera;

pub struct SettleTownCameraPlugin;

impl Plugin for SettleTownCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_camera);
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), OverheadMapCamera));
}

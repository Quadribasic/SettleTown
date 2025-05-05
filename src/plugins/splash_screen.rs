use std::time::Duration;

#[cfg(any(target_os = "macos", target_os = "linux"))]
use bevy::window::CompositeAlphaMode;
use bevy::{
    app::Plugin,
    prelude::*,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

use crate::states::game_states::GameState;

// #[derive(Component)]
// pub struct LoadingSplashScreenWindow;

/// A marker component for the splash screen sprite.
#[derive(Component)]
pub struct SplashScreenSprite;

pub struct SettleTownLoadingSplashScreenPlugin;

impl SettleTownLoadingSplashScreenPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2dBundle::default());
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("custom/screens/SplashScreenSunset.png"),
                ..default()
            },
            SplashScreenSprite,
        ));
    }
}

impl Plugin for SettleTownLoadingSplashScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SettleTown Splash Screen".into(),
                name: Some("com.quadribasic.settletown.splash".into()),

                // Setting `transparent` allows the `ClearColor`'s alpha value to take
                // effect
                transparent: true,
                // Disabling window decorations to make it feel more like a widget than a
                // window
                decorations: false,

                // In MacOS, windows are expected to have alpha postmultiplied
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,

                // In Linux, windows are expected to have alpha premultiplied
                #[cfg(target_os = "linux")]
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,

                // XXX_jwir3: What about Windows systems?
                ..Default::default()
            }),
            ..Default::default()
        }))
        // The app will start in the default state of `LoadingSplashScreen`
        .init_state::<GameState>()
        // ClearColor must have 0 alpha, otherwise some color will bleed through
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, SettleTownLoadingSplashScreenPlugin::setup);
    }
}

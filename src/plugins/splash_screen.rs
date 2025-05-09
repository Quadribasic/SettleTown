#[cfg(any(target_os = "macos", target_os = "linux"))]
use bevy::window::CompositeAlphaMode;
use bevy::{
    app::Plugin,
    prelude::*,
    render::camera::RenderTarget,
    window::{
        Cursor, InternalWindowState, Window, WindowPosition, WindowRef, WindowResizeConstraints,
    },
};

use crate::states::game_states::GameState;

/// A marker component for the splash screen sprite.
#[derive(Component)]
pub struct SplashScreenSprite;

/// A marker component for the splash screen window.
#[derive(Component)]
pub struct SplashScreenWindow;

pub struct SettleTownLoadingSplashScreenPlugin;

impl SettleTownLoadingSplashScreenPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Create the splash screen window
        let window = Window {
            title: "SettleTown Splash Screen".into(),
            name: Some("com.quadribasic.settletown.splash".into()),
            transparent: true,
            decorations: false,
            visible: true,
            focused: true,
            present_mode: bevy::window::PresentMode::AutoVsync,
            mode: bevy::window::WindowMode::Windowed,
            position: WindowPosition::default(),
            resolution: (800.0, 600.0).into(),
            resize_constraints: WindowResizeConstraints::default(),
            resizable: false,
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            window_level: bevy::window::WindowLevel::Normal,
            cursor: Cursor::default(),
            internal: InternalWindowState::default(),
            ime_enabled: false,
            prevent_default_event_handling: false,
            fit_canvas_to_parent: false,
            canvas: None,

            #[cfg(target_os = "macos")]
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,

            #[cfg(target_os = "linux")]
            composite_alpha_mode: CompositeAlphaMode::PreMultiplied,

            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            composite_alpha_mode: bevy::window::CompositeAlphaMode::Auto,

            // XXX_jwir3: What about Windows systems?
            ..Default::default()
        };

        // Spawn the window with a marker component
        let window_entity = commands
            .spawn((
                window,
                SplashScreenWindow,
                GlobalTransform::default(),
                InheritedVisibility::default(),
                Visibility::default(),
            ))
            .id();

        // Setup the camera and sprite as children of the window
        let camera_bundle = Camera2dBundle {
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(window_entity)),
                ..default()
            },
            ..default()
        };
        let camera_entity = commands.spawn(camera_bundle).id();
        commands.entity(window_entity).add_child(camera_entity);

        // Load the texture and wait for it to be ready
        let texture_handle = asset_server.load("custom/screens/SplashScreenSunset.png");

        let sprite_entity = commands
            .spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(1.0)),
                    ..default()
                },
                SplashScreenSprite,
            ))
            .id();
        commands.entity(window_entity).add_child(sprite_entity);
    }

    fn check_texture_loaded(
        mut texture_events: EventReader<AssetEvent<Image>>,
        texture_handles: Query<&Handle<Image>, With<SplashScreenSprite>>,
    ) {
        for event in texture_events.read() {
            for handle in texture_handles.iter() {
                match event {
                    AssetEvent::LoadedWithDependencies { id } if id == &handle.id() => {
                        info!("Splash screen texture loaded successfully");
                    }
                    AssetEvent::Unused { id } if id == &handle.id() => {
                        error!("Splash screen texture failed to load");
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Plugin for SettleTownLoadingSplashScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameState>()
            .insert_resource(ClearColor(Color::NONE))
            .add_systems(Startup, SettleTownLoadingSplashScreenPlugin::setup)
            .add_systems(
                Update,
                SettleTownLoadingSplashScreenPlugin::check_texture_loaded,
            );
    }
}

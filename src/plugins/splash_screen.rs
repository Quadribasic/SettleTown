#[cfg(any(target_os = "macos", target_os = "linux"))]
use bevy::window::CompositeAlphaMode;
use bevy::{
    app::Plugin,
    prelude::*,
    render::camera::RenderTarget,
    sprite::Anchor,
    window::{
        CursorOptions, InternalWindowState, Window, WindowPosition, WindowRef,
        WindowResizeConstraints,
    },
};

use crate::states::game_states::GameState;

/// A marker component for the splash screen sprite.
#[derive(Component)]
pub struct SplashScreenSprite;

/// A marker component for the splash screen text.
#[derive(Component)]
pub struct SplashScreenText;

/// A marker component for the splash screen textmark.
#[derive(Component)]
pub struct SplashScreenTextmark;

/// A marker component for the splash screen window.
#[derive(Component)]
pub struct SplashScreenWindow;

/// An event indicating that the lower text on the splash screen should be changed.
#[derive(Event)]
pub struct SplashScreenTextUpdateEvent(pub String);

pub struct SettleTownLoadingSplashScreenPlugin;

impl SettleTownLoadingSplashScreenPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("Setting up the splash screen");
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
            cursor_options: CursorOptions::default(),
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
        let camera_bundle = (
            Camera2d,
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(window_entity)),
                ..default()
            },
        );

        let camera_entity = commands.spawn(camera_bundle).id();
        commands.entity(window_entity).add_child(camera_entity);

        // Add UI camera
        let ui_camera_bundle = (
            Camera2d,
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(window_entity)),
                order: 1,
                ..default()
            },
        );
        let ui_camera_entity = commands.spawn(ui_camera_bundle).id();

        commands.entity(window_entity).add_child(ui_camera_entity);

        // Load the texture and wait for it to be ready
        let texture_handle = asset_server.load("custom/screens/SplashScreenSunset.png");

        let sprite_entity = commands
            .spawn((
                Sprite {
                    image: texture_handle.clone(),
                    ..default()
                },
                SplashScreenSprite,
            ))
            .id();
        commands.entity(window_entity).add_child(sprite_entity);

        // Add a textmark at the center top of the splash screen
        let textmark_handle: Handle<Image> = asset_server.load("custom/screens/Textmark.png");

        let textmark_entity = commands
            .spawn((
                Sprite {
                    image: textmark_handle.clone(),
                    ..default()
                },
                GlobalTransform::from_xyz(0.0, 125.0, 1.0),
                SplashScreenTextmark,
            ))
            .id();

        commands.entity(window_entity).add_child(textmark_entity);

        // Add loading text at the bottom of the splash screen
        let text_entity = commands
            .spawn((
                SplashScreenText,
                Text2d::new("Creating game world..."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor::WHITE,
                TextLayout::new_with_justify(JustifyText::Left),
                GlobalTransform::from_xyz(-240.0, -200.0, 1.0),
                Anchor::BottomLeft,
            ))
            .id();
        commands.entity(window_entity).add_child(text_entity);
    }

    fn check_texture_loaded(
        mut texture_events: EventReader<AssetEvent<Image>>,
        mut sprite_query: Query<&mut Sprite, With<SplashScreenSprite>>,
    ) {
        for event in texture_events.read() {
            for sprite in sprite_query.iter_mut() {
                match event {
                    AssetEvent::LoadedWithDependencies { id } if id == &sprite.image.id() => {
                        info!("Splash screen texture loaded successfully");
                    }
                    AssetEvent::Unused { id } if id == &sprite.image.id() => {
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
            .add_event::<SplashScreenTextUpdateEvent>()
            .add_systems(Startup, SettleTownLoadingSplashScreenPlugin::setup);
        app.add_systems(
            Update,
            (
                SettleTownLoadingSplashScreenPlugin::check_texture_loaded,
                update_splash_screen_text,
            ),
        );
        app.add_systems(
            OnExit(GameState::LoadingSplashScreen),
            despawn_splash_screen,
        );
    }
}

fn update_splash_screen_text(
    mut text_events: EventReader<SplashScreenTextUpdateEvent>,
    mut text_query: Query<&mut Text2d, With<SplashScreenText>>,
) {
    for event in text_events.read() {
        if let Ok(mut text) = text_query.single_mut() {
            text.0 = event.0.clone();
        }
    }
}

fn despawn_splash_screen(
    mut commands: Commands,
    splash_screen_window: Query<Entity, With<SplashScreenWindow>>,
) {
    // Despawn the splash screen window
    if let Ok(window_entity) = splash_screen_window.single() {
        info!(
            "Despawning splash screen window (Entity: {:?})",
            window_entity
        );

        // Despawn all children of this entity
        commands.entity(window_entity).remove::<Children>();

        // Despawn the window entity
        commands.entity(window_entity).despawn();
    } else {
        error!("Splash screen window was not found");
    }
}

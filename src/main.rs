use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod plugins;

use plugins::input::SettleTownInputPlugin;

const WINDOW_WIDTH: i32 = 1408;
const WINDOW_HEIGHT: i32 = 768;

// #[derive(Component)]
// struct Person;

// #[derive(Component)]
// struct Name(String);

// #[derive(Resource)]
// struct GreetTimer(Timer);

// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name("Elaina Proctor".to_string())));
//     commands.spawn((Person, Name("Renzo Hume".to_string())));
//     commands.spawn((Person, Name("Zayna Nieves".to_string())));
// }

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     // update our timer with the time elapsed since the last update
//     // if that caused the timer to finish, we say hello to everyone
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// fn update_people(mut query: Query<&mut Name, With<Person>>) {
//     for mut name in &mut query {
//         if name.0 == "Elaina Proctor" {
//             name.0 = "Elaina Hume".to_string();
//             break; // We don't need to change any other names.
//         }
//     }
// }

/// SettleTown is a game where you get to found a town in the old west. It uses Bevy, a game engine
/// for Rust. The coordinate system is a right-handed coordinate system, with the origin in the
/// center of the viewport.
/// When using the UiPlugin, the origin is the top left of the screen.
fn main() {
    // let args = Args::parse();

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SettleTown".into(),
                    name: Some("com.quadribasic.settletown.main".into()),
                    resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                    // present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    // window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(SettleTownInputPlugin)
        .add_systems(
            Update,
            (
                // change_title,
                // toggle_theme,
                // toggle_cursor,
                // toggle_vsync,
                // toggle_window_controls,
                // cycle_cursor_icon,
                // switch_level,
                make_window_visible,
            ),
        )
        .run();
}

fn make_window_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}

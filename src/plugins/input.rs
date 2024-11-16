use bevy::prelude::*;

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyQ) {
        info!("User requested quitting the application from the keyboard");
        std::process::exit(0);
    }
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

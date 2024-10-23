use macroquad::prelude::*;

mod game;
mod graphics;
mod world;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 768;

fn window_config() -> Conf {
    Conf {
        window_title: "SettleTown".to_owned(),
        fullscreen: false,
        // high_dpi: true,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut engine = game::Engine::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

    'gameloop: loop {
        clear_background(BLACK);

        engine.draw_map();

        next_frame().await;

        if is_key_down(KeyCode::Escape) {
            break 'gameloop;
        }
    }
}

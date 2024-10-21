use macroquad::prelude::*;
mod graphics;

fn window_config() -> Conf {
    Conf {
        window_title: "SettleTown".to_owned(),
        fullscreen: false,
        // high_dpi: true,
        window_width: 1200,
        window_height: 1200,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let renderer = graphics::Renderer::new();

    'gameloop: loop {
        clear_background(BLACK);

        renderer.draw();

        next_frame().await;

        if is_key_down(KeyCode::Escape) {
            break 'gameloop;
        }
    }
}

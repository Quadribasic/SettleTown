use clap::{command, Parser};
use graphics::Renderer;
use log::error;
use logger::setup_logger;
use macroquad::prelude::*;

mod assets;
mod graphics;
mod logger;
mod world;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 768;

fn window_config() -> Conf {
    Conf {
        window_title: "SettleTown".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

/// SettleTown is a game where you get to found a town in the old west.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to a map file to load, if one has been previously saved.
    #[arg(short, long)]
    map_file: Option<String>,
}

#[macroquad::main(window_config)]
async fn main() {
    match setup_logger() {
        Ok(()) => (),
        Err(e) => {
            error!("Unable to initialize logging with fern: {:?}", e);
        }
    }

    let args = Args::parse();

    let mut renderer =
        Renderer::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, args.map_file).await;

    'gameloop: loop {
        clear_background(BLACK);

        renderer.draw_world().await;

        next_frame().await;

        if is_key_down(KeyCode::Escape) {
            break 'gameloop;
        }
    }
}

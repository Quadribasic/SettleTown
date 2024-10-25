use std::{fs, path::Path};

use colored::Colorize;
use graphics::Renderer;
use log::error;
use macroquad::prelude::*;

mod assets;
mod graphics;
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

fn setup_logger() -> Result<(), fern::InitError> {
    let log_dir = Path::new("log");
    if !log_dir.exists() {
        fs::create_dir(log_dir)?;
    }

    let mut log_file = log_dir.join("debug.log");
    if log_file.exists() {
        fs::remove_file(log_file)?;
    }

    log_file = log_dir.join("debug.log");

    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        record.level().to_string(),
                        message
                    ))
                })
                .level(log::LevelFilter::Debug)
                // .chain(std::io::stdout())
                .chain(fern::log_file(&log_file)?),
        )
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    let colored_record_level = match record.level() {
                        log::Level::Debug => record.level().to_string().blue(),
                        log::Level::Info => record.level().to_string().blue(),
                        log::Level::Trace => record.level().to_string().purple(),
                        log::Level::Warn => record.level().to_string().yellow(),
                        log::Level::Error => record.level().to_string().red(),
                    };

                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        colored_record_level,
                        message
                    ))
                })
                .level(log::LevelFilter::Debug)
                .chain(std::io::stdout()),
        )
        .apply()?;

    Ok(())
}

#[macroquad::main(window_config)]
async fn main() {
    match setup_logger() {
        Ok(()) => (),
        Err(e) => {
            error!("Unable to initialize logging with fern: {:?}", e);
        }
    }

    let mut renderer = Renderer::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).await;

    'gameloop: loop {
        clear_background(BLACK);

        renderer.draw_world().await;

        next_frame().await;

        if is_key_down(KeyCode::Escape) {
            break 'gameloop;
        }
    }
}

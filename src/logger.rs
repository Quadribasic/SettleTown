use std::{fs, path::Path};

use colored::Colorize;

pub fn setup_logger() -> Result<(), fern::InitError> {
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

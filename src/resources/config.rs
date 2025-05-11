use std::{env, path::PathBuf};

use bevy::{log::info, prelude::Resource};
use directories::ProjectDirs;
use serde::Deserialize;

/// A configuration resource that should available whenever the game is running.
/// Serialized and deserialized from a config file using serde in toml format.
#[derive(Debug, Resource)]
pub struct SettleTownConfig {
    config_dir: PathBuf,
    base_config: BaseConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "config")]
struct BaseConfig {
    pub world: WorldConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "world")]
struct WorldConfig {
    pub enable_aabbs: bool,
}

impl SettleTownConfig {
    pub fn clobber_from_default() -> Self {
        let config_dir = SettleTownConfig::get_config_directory();

        // Copy the default configuration file, overwriting if it does exist
        let config_file = config_dir.join("config.toml");
        let default_config = std::path::Path::new("assets/config/default.toml");
        if default_config.exists() {
            std::fs::copy(default_config, &config_file)
                .expect("Failed to copy default configuration file");
            info!("Copied default configuration file to {:?}", config_file);
        } else {
            info!(
                "Default configuration file not found at {:?}",
                default_config
            );
        }

        SettleTownConfig::parse_config_file(config_dir, &config_file)
    }

    pub fn create_from_default() -> Self {
        let config_dir = SettleTownConfig::get_config_directory();

        // Copy the default configuration file if it doesn't exist
        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {
            let default_config = std::path::Path::new("assets/config/default.toml");
            if default_config.exists() {
                std::fs::copy(default_config, &config_file)
                    .expect("Failed to copy default configuration file");
                info!("Copied default configuration file to {:?}", config_file);
            } else {
                info!(
                    "Default configuration file not found at {:?}",
                    default_config
                );
            }
        }

        SettleTownConfig::parse_config_file(config_dir, &config_file)
    }

    pub fn get_config_directory() -> PathBuf {
        let project_dirs = ProjectDirs::from(
            &env::var("CARGO_PKG_QUALIFIER").unwrap_or_else(|_| "com".to_string()),
            &env::var("CARGO_PKG_ORGANIZATION").unwrap_or_else(|_| "quadribasic".to_string()),
            &env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "SettleTown".to_string()),
        )
        .expect("Failed to get project directories");

        info!("Project directories are: {:?}", project_dirs);

        let config_dir = project_dirs.config_dir().to_path_buf();

        // Create the config directory if it doesn't exist
        std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        config_dir
    }

    pub fn parse_config_file(config_dir: PathBuf, config_file: &PathBuf) -> Self {
        // Parse the configuration file
        let config =
            std::fs::read_to_string(&config_file).expect("Failed to read configuration file");
        let config: BaseConfig =
            toml::from_str(&config).expect("Failed to parse configuration file");

        info!("Found configuration: {:?}", config);

        Self {
            config_dir,
            base_config: config,
        }
    }
}

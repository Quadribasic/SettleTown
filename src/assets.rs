use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use log::error;
use macroquad::{file::load_file, texture::Texture2D};
use serde::{Deserialize, Serialize};

use crate::graphics::TileType;

pub struct AssetLoader {
    asset_json_path: String,
}

impl AssetLoader {
    pub fn new(asset_json_path: &str) -> Self {
        let loader = Self {
            asset_json_path: asset_json_path.to_string(),
        };

        match loader.load_texture_atlas_from_file() {
            Ok(_t) => (),
            Err(e) => {
                error!("Unable to load textures from file due to: {:?}", e);
            }
        };

        loader
    }

    pub fn load_texture_atlas_from_file(&self) -> Result<TextureAtlas, Box<dyn Error>> {
        let file = File::open(&self.asset_json_path)?;

        let reader = BufReader::new(file);

        let texture_atlas = serde_json::from_reader(reader)?;

        Ok(AssetLoader::adjust_texture_atlas_paths_for_atlas_file(
            texture_atlas,
            &self.asset_json_path,
        ))
    }

    /// Adjust all paths for this asset loader to be relative to the path from which an
    /// 'assets.json' file is located.
    pub fn adjust_texture_atlas_paths_for_atlas_file(
        texture_atlas: TextureAtlas,
        asset_json_path: &str,
    ) -> TextureAtlas {
        let path = Path::new(asset_json_path).parent().unwrap();

        let mut new_texture_atlas = TextureAtlas { tiles: Vec::new() };

        for td in texture_atlas.tiles {
            let adjusted_path = path.join(td.file);
            let adjusted_path_string = adjusted_path.to_str().unwrap().to_string();
            new_texture_atlas.tiles.push(TileDescription {
                name: td.name,
                file: adjusted_path_string,
                cell: td.cell,
                cell_size: td.cell_size,
            });
        }

        new_texture_atlas
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CellSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TileDescription {
    pub name: String,
    pub file: String,
    pub cell: Cell,
    pub cell_size: CellSize,
}

impl TileDescription {
    pub fn get_start_location(&self) -> (u32, u32) {
        (
            self.cell_size.width * self.cell.x as u32,
            self.cell_size.height * self.cell.y as u32,
        )
    }
}

pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}

impl TextureManager {
    pub async fn get_texture(&mut self, file_path: &str) -> Texture2D {
        if !self.textures.contains_key(file_path) {
            let file_data: Vec<u8> = match load_file(file_path).await {
                Ok(x) => x,
                Err(e) => {
                    println!(
                        "ERROR: Unable to load file from path: {}. Error was: {}",
                        file_path, e
                    );
                    vec![]
                }
            };

            let texture = Texture2D::from_file_with_format(&file_data, None);
            self.textures.insert(file_path.to_string(), texture);
        }

        return self.textures.get(file_path).unwrap().clone();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TextureAtlas {
    pub tiles: Vec<TileDescription>,
}

impl TextureAtlas {
    pub fn get_tile_description_for_tile_type(
        &self,
        tile_type: &TileType,
    ) -> Option<TileDescription> {
        let found_tile_description = self
            .tiles
            .clone()
            .into_iter()
            .filter(|td| td.name == *tile_type)
            .next()
            .map(|td| td.clone());
        found_tile_description
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::TileType;

    use super::*;

    #[test]
    fn it_should_be_able_to_load_a_texture_atlas() {
        let asset_loader = AssetLoader::new("assets/assets.json");
        let texture_atlas_result = asset_loader.load_texture_atlas_from_file();

        assert!(texture_atlas_result.is_ok());

        let texture_atlas = texture_atlas_result.unwrap();

        assert_eq!(texture_atlas.tiles.len(), 2);

        let mut tile_iter = texture_atlas.tiles.iter();
        let tile1 = tile_iter.next().unwrap();

        assert_eq!(tile1.name, TileType::Grass);
        assert_eq!(tile1.file, "dummy-lpc/tiles/grass.png");
        assert_eq!(tile1.cell, Cell { x: 1, y: 3 });
        assert_eq!(
            tile1.cell_size,
            CellSize {
                width: 32,
                height: 32
            }
        );
    }
}

use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use log::{debug, error};
use macroquad::{file::load_file, texture::Texture2D};
use serde::{Deserialize, Serialize};

use crate::graphics::TileType;

use rand::prelude::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Alternative {
    pub target: String,
    pub percentage: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CellSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TileDescription {
    pub name: String,
    pub file: String,
    pub cell: Cell,
    pub cell_size: CellSize,
    pub alternative: Option<Alternative>,
}

impl TileDescription {
    pub fn get_start_location(&self) -> (u32, u32) {
        (
            self.cell_size.height * self.cell.column as u32,
            self.cell_size.width * self.cell.row as u32,
        )
    }

    pub fn is_alternative(&self) -> bool {
        self.alternative.is_some()
    }
}

/// A loader for assets from a JSON file.
pub struct AssetLoader {
    asset_json_path: String,
    texture_atlas: Option<TextureAtlas>,
}

impl AssetLoader {
    pub fn new(asset_json_path: &str) -> Self {
        let mut loader = Self {
            asset_json_path: asset_json_path.to_string(),
            texture_atlas: None,
        };

        match loader.load_texture_atlas_from_file() {
            Ok(_) => {}
            Err(_e) => {
                error!("Texture atlas is uninitialized. Default textures will be used for textures from this atlas.");
            }
        }

        loader
    }

    fn load_texture_atlas_from_file(&mut self) -> Result<(), Box<dyn Error>> {
        if self.texture_atlas.is_some() {
            debug!("Texture atlas is already some value.");
            return Ok(());
        }

        let file = File::open(&self.asset_json_path)?;
        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(t) => {
                let adjusted_texture_atlas = AssetLoader::adjust_texture_atlas_paths_for_atlas_file(
                    t,
                    &self.asset_json_path,
                );
                self.texture_atlas = Some(adjusted_texture_atlas);
            }
            Err(e) => {
                error!("Encountered issue when loading json from reader: {:?}", e);
                self.texture_atlas = None;
            }
        }

        Ok(())
    }

    /// Adjust all paths for this asset loader to be relative to the path from which an
    /// 'assets.json' file is located.
    fn adjust_texture_atlas_paths_for_atlas_file(
        texture_atlas: TextureAtlas,
        asset_json_path: &str,
    ) -> TextureAtlas {
        let path = Path::new(asset_json_path).parent().unwrap();

        let mut new_texture_atlas = TextureAtlas {
            tiles: Vec::new(),
            alternates: HashMap::new(),
        };

        for td in texture_atlas.tiles {
            let adjusted_path = path.join(td.file);
            let adjusted_path_string = adjusted_path.to_str().unwrap().to_string();
            new_texture_atlas.tiles.push(TileDescription {
                name: td.name,
                file: adjusted_path_string,
                cell: td.cell,
                cell_size: td.cell_size,
                alternative: td.alternative,
            });
        }

        new_texture_atlas
    }

    pub fn get_texture_atlas(&self) -> Option<TextureAtlas> {
        self.texture_atlas.clone()
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextureAtlas {
    pub tiles: Vec<TileDescription>,

    #[serde(skip)]
    pub alternates: HashMap<TileType, Vec<TileDescription>>,
}

impl TextureAtlas {
    pub fn get_tile_description_for_tile_type(
        &mut self,
        tile_type: &TileType,
    ) -> Option<TileDescription> {
        let found_tile_description = self
            .tiles
            .clone()
            .into_iter()
            .filter(|td| td.name == *tile_type)
            .next()
            .map(|td| td.clone());

        // Apply any alternatives, if there are any.
        let alternatives = self.get_alternatives_for(&tile_type);
        if alternatives.len() > 0 {
            let mut rng = rand::thread_rng();
            for alt in alternatives {
                let random_number: f32 = rng.gen();
                if random_number < alt.alternative.clone().unwrap().percentage {
                    return Some(alt);
                }
            }
        }

        found_tile_description
    }

    pub fn get_alternatives_for(&mut self, tile_type: &TileType) -> Vec<TileDescription> {
        if self.alternates.contains_key(&tile_type) {
            return self.alternates.get(&tile_type).unwrap().clone();
        }

        let alternatives: Vec<TileDescription> = self
            .tiles
            .iter()
            .filter(|td| {
                td.is_alternative() && td.alternative.clone().unwrap().target == *tile_type
            })
            .map(|td| td.clone())
            .collect();

        self.alternates
            .insert(tile_type.clone(), alternatives.clone());

        alternatives
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init_tests() {
    use crate::logger::setup_logger;

    setup_logger().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::graphics::TileType;

    use super::*;

    #[test]
    fn it_should_be_able_to_load_a_texture_atlas() {
        let asset_loader = AssetLoader::new("assets/assets.json");
        assert!(asset_loader.texture_atlas.is_some());

        let texture_atlas = asset_loader.get_texture_atlas().unwrap();
        assert_eq!(texture_atlas.tiles.len(), 5);

        let mut tile_iter = texture_atlas.tiles.iter();
        let tile1 = tile_iter.next().unwrap();

        assert_eq!(tile1.name, TileType::Grass);
        assert_eq!(tile1.file, "assets/dummy-lpc/tiles/grass.png");
        assert_eq!(tile1.cell, Cell { row: 3, column: 1 });
        assert_eq!(
            tile1.cell_size,
            CellSize {
                width: 32,
                height: 32
            }
        );
    }

    #[test]
    fn it_should_be_able_to_contain_alternatives() {
        let asset_loader = AssetLoader::new("assets/assets.json");
        assert!(asset_loader.texture_atlas.is_some());

        let texture_atlas = asset_loader.get_texture_atlas().unwrap();
        assert_eq!(texture_atlas.tiles.len(), 5);

        let mut tile_iter = texture_atlas.tiles.iter();
        let _tile1 = tile_iter.next().unwrap();
        let _tile2 = tile_iter.next().unwrap();
        let tile3 = tile_iter.next().unwrap();

        assert_eq!(tile3.name, TileType::GrassAlternate1);
        assert_eq!(tile3.file, "assets/dummy-lpc/tiles/grass.png");
        assert_eq!(tile3.cell, Cell { row: 5, column: 0 });
        assert_eq!(
            tile3.cell_size,
            CellSize {
                width: 32,
                height: 32,
            }
        );

        assert!(tile3.is_alternative());
    }

    #[test]
    fn it_should_have_three_alternatives_for_grass() {
        let asset_loader = AssetLoader::new("assets/assets.json");
        assert!(asset_loader.texture_atlas.is_some());

        let mut texture_atlas = asset_loader.get_texture_atlas().unwrap();
        assert_eq!(texture_atlas.tiles.len(), 5);

        let alternatives = texture_atlas.get_alternatives_for(&TileType::Grass);
        assert_eq!(alternatives.len(), 3);

        let first_alt = alternatives.iter().next().unwrap();
        assert_eq!(first_alt.name, "GrassAlternate1".to_string());
        assert_eq!(first_alt.cell, Cell { row: 5, column: 0 });
        assert_eq!(
            first_alt.cell_size,
            CellSize {
                width: 32,
                height: 32
            }
        );
    }
}

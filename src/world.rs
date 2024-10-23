use std::collections::HashMap;

use macroquad::texture::Texture2D;

use crate::graphics::{TILE_SIZE_X, TILE_SIZE_Y};

#[derive(Debug)]
pub struct WorldError {
    message: String,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum WorldBlockType {
    Desert,
    Grass,
}

pub struct World {
    window_width: f32,
    window_height: f32,
    tile_types: Vec<Vec<WorldBlockType>>,
    block_textures: HashMap<WorldBlockType, Texture2D>,
}

impl World {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let block_textures = HashMap::new();

        let mut instance = Self {
            window_width,
            window_height,
            tile_types: vec![],
            block_textures,
        };

        let world_size = instance.get_world_size_in_tiles();
        for i in 0..world_size.0 {
            let mut tile_row: Vec<WorldBlockType> = Vec::new();

            for j in 0..world_size.1 {
                if i > 12 && i < 16 {
                    tile_row.push(WorldBlockType::Grass);
                } else {
                    tile_row.push(WorldBlockType::Desert);
                }
            }

            instance.tile_types.push(tile_row);
        }

        instance
    }

    pub fn get_world_size_in_tiles(&self) -> (usize, usize) {
        let world_width_tiles = (self.window_width / TILE_SIZE_X) as usize;
        let world_height_tiles = (self.window_height / TILE_SIZE_Y) as usize;

        (world_width_tiles, world_height_tiles)
    }

    pub fn get_tile_type_at(&self, x: usize, y: usize) -> Result<WorldBlockType, WorldError> {
        let world_size = self.get_world_size_in_tiles();

        if x > world_size.0 || y > world_size.1 {
            return Err(WorldError {
                message: "x or y position exceeds maximum size of world".to_string(),
            });
        }

        Ok(self.tile_types[x][y])
    }

    pub fn get_texture_for(&mut self, block_type: WorldBlockType) -> Texture2D {
        if self.block_textures.contains_key(&block_type) {
            self.block_textures.get(&block_type).unwrap().clone()
        } else {
            let texture = match block_type {
                WorldBlockType::Desert => Texture2D::from_file_with_format(
                    include_bytes!("../assets/desert-tiles/plain-desert-tile-1.png"),
                    None,
                ),
                WorldBlockType::Grass => Texture2D::from_file_with_format(
                    include_bytes!("../assets/dummy-lpc/tiles/grass-basic-1.png"),
                    None,
                ),
            };

            self.block_textures.insert(block_type, texture.clone());

            texture
        }
    }
}

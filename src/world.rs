use std::fmt::Display;

use crate::graphics::{TileType, TILE_SIZE_X, TILE_SIZE_Y};

#[derive(Debug)]
pub struct WorldError {
    message: String,
}

impl Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

/// The World represents the logical items in the game world. This includes the map or base ground
/// layer, the buildings, obstacles, terrain, and characters (both player and non-player).
pub struct World {
    window_width: f32,
    window_height: f32,
    tile_types: Vec<Vec<TileType>>,
}

impl World {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let mut instance = Self {
            window_width,
            window_height,
            tile_types: vec![],
        };

        let world_size = instance.get_world_size_in_tiles();
        for i in 0..world_size.0 {
            let mut tile_row: Vec<TileType> = Vec::new();

            for _j in 0..world_size.1 {
                if i > 12 && i < 16 {
                    tile_row.push(TileType::Grass);
                } else {
                    tile_row.push(TileType::Dirt);
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

    pub fn get_tile_type_at(&self, x: usize, y: usize) -> Result<TileType, WorldError> {
        let world_size = self.get_world_size_in_tiles();

        if x > world_size.0 || y > world_size.1 {
            return Err(WorldError {
                message: "x or y position exceeds maximum size of world".to_string(),
            });
        }

        Ok(self.tile_types[x][y].clone())
    }
}

use std::fmt::Display;

use crate::graphics::{RenderableTile, Renderer, TileType, TILE_SIZE_X, TILE_SIZE_Y};

#[derive(Debug)]
pub struct WorldError {
    message: String,
}

impl Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

#[derive(Clone, Debug)]
/// The World represents the logical items in the game world. This includes the map or base ground
/// layer, the buildings, obstacles, terrain, and characters (both player and non-player).
pub struct World {
    window_width: f32,
    window_height: f32,
    tile_types: Vec<Vec<TileType>>,
    pub world_tiles: Option<Vec<Vec<RenderableTile>>>,
}

impl World {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let mut instance = Self {
            window_width,
            window_height,
            tile_types: vec![],
            world_tiles: None,
        };

        let world_size = instance.get_world_size_in_tiles();
        for _i in 0..world_size.0 {
            let mut tile_row: Vec<TileType> = Vec::new();

            // Every so often (1/10 chance), put an alternate grass tile.
            for _j in 0..world_size.1 {
                tile_row.push(TileType::Grass);
            }

            instance.tile_types.push(tile_row);
        }

        instance
    }

    pub async fn draw(&mut self, renderer: &mut Renderer) {
        let world_size = self.get_world_size_in_tiles();

        if self.world_tiles.is_none() {
            // debug!("Generating tiles");
            let mut world_tiles: Vec<Vec<RenderableTile>> = Vec::new();
            for i in 0..world_size.0 {
                let mut world_tile_row: Vec<RenderableTile> = Vec::new();
                for j in 0..world_size.1 {
                    let tile_type = self.get_tile_type_at(i, j).unwrap();
                    let renderable_tile = renderer.get_renderable_tile_for(&tile_type).await;
                    world_tile_row.push(renderable_tile);
                }

                world_tiles.push(world_tile_row);
            }

            self.world_tiles = Some(world_tiles);
        }

        // debug!("Self.world_tiles is now: {:?}", self.world_tiles);
        let world_tiles = self.world_tiles.as_ref().unwrap();
        for i in 0..world_size.0 {
            for j in 0..world_size.1 {
                let tile = world_tiles[i][j].clone();
                tile.draw((i as f32) * TILE_SIZE_X, (j as f32) * TILE_SIZE_Y);
            }
        }
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

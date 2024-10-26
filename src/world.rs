use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use log::{debug, warn};
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<TileType>>,
}

#[derive(Clone, Debug)]
/// The World represents the logical items in the game world. This includes the map or base ground
/// layer, the buildings, obstacles, terrain, and characters (both player and non-player).
pub struct World {
    window_width: f32,
    window_height: f32,
    map: Option<Map>,
    pub world_tiles: Option<Vec<Vec<RenderableTile>>>,
}

impl World {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let mut instance = Self {
            window_width,
            window_height,
            map: None,
            world_tiles: None,
        };

        let mut tile_types = vec![];
        let world_size = instance.get_world_size_in_tiles();
        for _i in 0..world_size.0 {
            let mut tile_row: Vec<TileType> = Vec::new();

            // Every so often (1/10 chance), put an alternate grass tile.
            for _j in 0..world_size.1 {
                tile_row.push(TileType::Grass);
            }

            tile_types.push(tile_row);
        }

        instance.map = Some(Map {
            width: world_size.0,
            height: world_size.1,
            tiles: tile_types,
        });

        instance
    }

    pub fn new_from_map_json(window_width: f32, window_height: f32, map_json: &str) -> Self {
        let map_result: Result<Map, _> = serde_json::from_str(map_json);
        let map;
        match map_result {
            Err(e) => {
                warn!(
                    "Encountered an error while reading map json. Map will be auto generated. Error was: {:?}",
                    e
                );
                return World::new(window_width, window_height);
            }
            Ok(m) => {
                map = m;
            }
        }

        let instance = Self {
            window_width,
            window_height,
            map: Some(map),
            world_tiles: None,
        };

        instance
    }

    pub fn new_from_map_file(
        window_width: f32,
        window_height: f32,
        map_file: Option<String>,
    ) -> Self {
        if map_file.is_none() {
            return World::new(window_width, window_height);
        }

        let map_file_unwrapped = map_file.unwrap();
        let map_file_path = Path::new(&map_file_unwrapped);

        if !map_file_path.exists() {
            warn!(
                "Map file path {:?} does not exist. Generating new world.",
                map_file_unwrapped
            );

            return World::new(window_width, window_height);
        }

        let file_result = File::open(map_file_path);
        let file: File;
        match file_result {
            Ok(f) => {
                file = f;
            }
            Err(e) => {
                warn!("Error opening map file: {:?}. Generating new world.", e);
                return World::new(window_width, window_height);
            }
        };
        let mut reader = BufReader::new(file);
        let mut buf: String = String::new();
        let read_result = reader.read_to_string(&mut buf);
        match read_result {
            Ok(b) => {
                debug!(
                    "Map file read ok. Read {} bytes from {}",
                    b, map_file_unwrapped
                );
            }
            Err(e) => {
                warn!(
                    "Map file read failed due to: {:?}. Generating new world.",
                    e
                );

                return World::new(window_width, window_height);
            }
        };

        return World::new_from_map_json(window_width, window_height, &buf);
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
        if self.map.is_none() {
            let world_width_tiles = (self.window_width / TILE_SIZE_X) as usize;
            let world_height_tiles = (self.window_height / TILE_SIZE_Y) as usize;

            (world_width_tiles, world_height_tiles)
        } else {
            let map = self.map.as_ref().unwrap();
            (map.width, map.height)
        }
    }

    pub fn get_tile_type_at(&self, x: usize, y: usize) -> Result<TileType, WorldError> {
        let world_size = self.get_world_size_in_tiles();

        if x > world_size.0 || y > world_size.1 {
            return Err(WorldError {
                message: "x or y position exceeds maximum size of world".to_string(),
            });
        }

        Ok(self.map.clone().unwrap().tiles[x][y].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_able_to_import_a_map_from_json_file() {
        let world = World::new_from_map_file(
            1200.,
            1200.,
            Some("assets/maps/default-map.json".to_string()),
        );
        assert!(world.map.is_some());

        let world_size = world.get_world_size_in_tiles();

        assert_eq!(world_size.0, 19);
        assert_eq!(world_size.1, 19);
    }
}

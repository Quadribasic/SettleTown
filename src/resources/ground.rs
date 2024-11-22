use bevy::prelude::Resource;
use bevy_ecs_tilemap::map::TilemapSize;
use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EdgeDirection {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

// #[derive(PartialEq, Clone)]
// pub struct GroundTileType {
//     texture_index: u32,
//     name: &'static str,
//     texture_path: &'static str,
// }

// impl GroundTileType {
//     const fn new(name: &'static str, texture_index: u32, texture_path: &'static str) -> Self {
//         Self {
//             name,
//             texture_index,
//             texture_path,
//         }
//     }
// }

// impl Into<u32> for GroundTileType {
//     fn into(self) -> u32 {
//         self.texture_index
//     }
// }

// pub const GRASS: GroundTileType = GroundTileType::new("Grass", 10, "dummy-lpc/tiles/grass.png");
// pub const GRASS_ALTERNATE1: GroundTileType =
//     GroundTileType::new("GrassAlternate1", 15, "dummy-lpc/tiles/grass.png");
// pub const GRASS_ALTERNATE2: GroundTileType =
//     GroundTileType::new("GrassAlternate2", 16, "dummy-lpc/tiles/grass.png");
// pub const GRASS_ALTERNATE3: GroundTileType =
//     GroundTileType::new("GrassAlternate3", 17, "dummy-lpc/tiles/grass.png");
// pub const DIRT: GroundTileType = GroundTileType::new("Dirt", 10, "dummy-lpc/tiles/dirt.png");

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum GroundTileType {
    Grass = 10,
    GrassEdgeTopLeft = 6,
    GrassEdgeTop = 7,
    GrassEdgeTopRight = 8,
    GrassEdgeLeft = 9,
    GrassEdgeRight = 11,
    GrassEdgeBottomLeft = 12,
    GrassEdgeBottom = 13,
    GrassEdgeBottomRight = 14,
    GrassAlternate1 = 15,
    GrassAlternate2 = 16,
    GrassAlternate3 = 17,
}

impl Into<u32> for GroundTileType {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Resource)]
pub struct Map {
    map_size: TilemapSize,
    ground_tiles: Vec<Vec<GroundTileType>>,
    default_ground_tile: GroundTileType,
}

impl Map {
    pub fn new(width: u32, height: u32, base_tile_type: GroundTileType) -> Self {
        let mut tiles = vec![];

        for _i in 0..width {
            let mut tile_row: Vec<GroundTileType> = vec![];
            for _j in 0..height {
                let tile = base_tile_type.clone();
                tile_row.push(tile);
            }

            tiles.push(tile_row);
        }

        Map {
            map_size: TilemapSize {
                x: width,
                y: height,
            },
            default_ground_tile: base_tile_type,
            ground_tiles: tiles,
        }
    }

    pub fn get_ground_type_at(&self, x: usize, y: usize) -> Option<GroundTileType> {
        match self.ground_tiles.get(x) {
            None => None,
            Some(val) => match val.get(y) {
                None => None,
                Some(ret_val) => Some(ret_val.clone()),
            },
        }
    }

    pub fn set_ground_type_at(&mut self, x: usize, y: usize, ground_type: GroundTileType) {
        self.ground_tiles[x][y] = ground_type;
    }

    pub fn get_map_size(&self) -> TilemapSize {
        self.map_size.clone()
    }

    pub fn get_default_ground_tile(&self) -> GroundTileType {
        self.default_ground_tile.clone()
    }
}

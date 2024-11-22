use bevy::prelude::Resource;
use bevy_ecs_tilemap::map::TilemapSize;

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

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum GroundTileType {
    GrassEdgeTopLeft = 12,
    GrassEdgeTop = 13,
    GrassEdgeTopRight = 14,
    GrassEdgeLeft = 18,
    Grass = 19,
    GrassEdgeRight = 20,
    Dirt = 22,
    GrassEdgeBottomLeft = 24,
    GrassEdgeBottom = 25,
    GrassEdgeBottomRight = 26,
    GrassAlternate1 = 30,
    GrassAlternate2 = 31,
    GrassAlternate3 = 32,
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

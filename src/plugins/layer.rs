use bevy::{app::Plugin, prelude::Resource};
use bevy_ecs_tilemap::TilemapPlugin;
use serde::{Deserialize, Serialize};

pub struct SettleTownLayerPlugin;

impl Plugin for SettleTownLayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TilemapPlugin);
        // app.add_systems(Startup, setup_map);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EdgeDirection {
    OutsideTopLeft,
    Top,
    OutsideTopRight,
    Right,
    OutsideBottomRight,
    Bottom,
    OutsideBottomLeft,
    Left,
}

#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum GroundTileType {
    GrassEdgeInsideCornerTopLeft = 1,
    GrassEdgeOutsideTopLeft = 12,
    GrassEdgeTop = 13,
    GrassEdgeOutsideTopRight = 14,
    RockyDirtEdgeOutsideTopLeft = 15,
    RockyDirtEdgeTop = 16,
    RockyDirtEdgeOutsideTopRight = 17,
    GrassEdgeLeft = 18,
    Grass = 19,
    GrassEdgeRight = 20,
    RockyDirtEdgeLeft = 21,
    RockyDirt = 22,
    RockyDirtEdgeRight = 23,
    GrassEdgeOutsideBottomLeft = 24,
    GrassEdgeBottom = 25,
    GrassEdgeOutsideBottomRight = 26,
    RockyDirtEdgeOutsideBottomLeft = 27,
    RockyDirtEdgeBottom = 28,
    RockyDirtEdgeOutsideBottomRight = 29,
    GrassAlternate1 = 30,
    GrassAlternate2 = 31,
    GrassAlternate3 = 32,
    RockyDirtAlternate1 = 33,
    RockyDirtAlternate2 = 34,
    RockyDirtAlternate3 = 35,
}

impl Into<u32> for GroundTileType {
    fn into(self) -> u32 {
        self as u32
    }
}

// impl RenderableTileType for GroundTileType {
//     fn as_u32(self) -> u32 {
//         self as u32
//     }
// }

// pub trait RenderableTileType: DynClone + Sync + Send {
//     fn as_u32(self) -> u32;
// }

/// A layer that can be rendered to the screen.
#[derive(Clone, Resource)]
pub struct Layer {
    /// The size of the layer, (height, width), in tiles.
    /// NOTE: This may need to be changed to pixels (or other units) in the future if we want to
    ///       support layers that are more than just 32x32 tiles.
    layer_size: (usize, usize),
    // tiles: Vec<Vec<Option<Arc<&dyn RenderableTileType>>>>,
    tiles: Vec<Vec<Option<GroundTileType>>>,
    default_tile: Option<GroundTileType>,
}

impl Layer {
    /// Initialize an empty `Layer` with a specified width and height.
    pub fn new(width: usize, height: usize) -> Self {
        let tiles: Vec<Vec<Option<GroundTileType>>> = vec![vec![None; width]; height];

        Self {
            layer_size: (width, height),
            tiles: tiles,
            default_tile: None,
        }
    }

    pub fn new_from_base_tile(width: usize, height: usize, base_tile: GroundTileType) -> Self {
        let mut tiles: Vec<Vec<Option<GroundTileType>>> = vec![];
        for _i in 0..width {
            let mut row: Vec<Option<GroundTileType>> = vec![];
            for _j in 0..height {
                let tile = base_tile.clone();
                row.push(Some(tile));
            }

            tiles.push(row);
        }

        Self {
            tiles: tiles,
            layer_size: (width, height),
            default_tile: Some(base_tile.clone()),
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.layer_size
    }

    pub fn get_tile_type_at(&self, x: usize, y: usize) -> Option<GroundTileType> {
        if let Some(row) = self.tiles.get(x) {
            if let Some(val) = row.get(y) {
                return val.clone();
            }
        }

        None
    }

    pub fn set_tile_type_at(&mut self, x: usize, y: usize, tile_type: GroundTileType) {
        self.tiles[x][y] = Some(tile_type.clone());
    }

    pub fn get_default_tile_type(&self) -> Option<GroundTileType> {
        self.default_tile.clone()
    }
}

use std::collections::HashMap;

use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    prelude::{Commands, Res, Resource},
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use mapgen::{DrunkardsWalk, MapBuilder as MapgenMapBuilder, TileType};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::resources::world::WorldCoordinates;

const TILE_MAP_TILE_WIDTH: f32 = 32.0;
const TILE_MAP_TILE_HEIGHT: f32 = 32.0;

pub struct SettleTownLayerPlugin {
    layer_width: usize,
    layer_height: usize,
}

impl SettleTownLayerPlugin {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            layer_width: width,
            layer_height: height,
        }
    }
}

impl Plugin for SettleTownLayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut layer_stack = LayerStack::new();
        layer_stack.add_layers(
            GroundLayerBuilder::new(self.layer_width, self.layer_height, GroundTileType::Grass)
                .add_edges_and_corner_tiles_for_base_tile(
                    GroundTileType::Grass,
                    vec![
                        (
                            EdgeDirection::OutsideTopLeft,
                            GroundTileType::GrassEdgeOutsideTopLeft,
                        ),
                        (EdgeDirection::Top, GroundTileType::GrassEdgeTop),
                        (
                            EdgeDirection::OutsideTopRight,
                            GroundTileType::GrassEdgeOutsideTopRight,
                        ),
                        (EdgeDirection::Right, GroundTileType::GrassEdgeRight),
                        (
                            EdgeDirection::OutsideBottomRight,
                            GroundTileType::GrassEdgeOutsideBottomRight,
                        ),
                        (EdgeDirection::Bottom, GroundTileType::GrassEdgeBottom),
                        (
                            EdgeDirection::OutsideBottomLeft,
                            GroundTileType::GrassEdgeOutsideBottomLeft,
                        ),
                        (EdgeDirection::Left, GroundTileType::GrassEdgeLeft),
                    ],
                )
                .generate_variegated_base(GroundTileType::RockyDirt)
                // .decorate_edge_tiles_differing_from_base(
                //     GroundTileType::Grass,
                //     (
                //         GroundTileType::GrassEdgeTop,
                //         GroundTileType::GrassEdgeRight,
                //         GroundTileType::GrassEdgeBottom,
                //         GroundTileType::GrassEdgeLeft,
                //     ),
                // )
                // .replace_inside_top_left_corners(
                //     GroundTileType::Grass,
                //     GroundTileType::GrassEdgeInsideCornerTopLeft,
                // )
                // Add alternatives last to simplify processing of tiles.
                .add_alternative_for(
                    GroundTileType::Grass,
                    GroundTileType::GrassAlternate1,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::Grass,
                    GroundTileType::GrassAlternate2,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::Grass,
                    GroundTileType::GrassAlternate3,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::RockyDirt,
                    GroundTileType::RockyDirtAlternate1,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::RockyDirt,
                    GroundTileType::RockyDirtAlternate2,
                    0.033,
                )
                .add_alternative_for(
                    GroundTileType::RockyDirt,
                    GroundTileType::RockyDirtAlternate3,
                    0.033,
                )
                .build(),
        );
        app.add_plugins(TilemapPlugin);
        app.insert_resource(layer_stack);
        app.insert_resource(WorldCoordinates::new(
            TILE_MAP_TILE_WIDTH,
            TILE_MAP_TILE_HEIGHT,
            self.layer_width,
            self.layer_height,
        ));
        app.add_systems(Startup, setup_rendering_of_layers);
    }
}

fn setup_rendering_of_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    layer_stack: Res<LayerStack>,
) {
    // info!("There are {} layers!", layer_stack.get_num_layers());
    for layer_num in 0..layer_stack.get_num_layers() {
        let layer = layer_stack.get_layer_at_level(layer_num).unwrap();
        let map_size = layer.get_size();
        let width = map_size.0 as u32;
        let height = map_size.1 as u32;
        let tilemap_size = TilemapSize {
            x: width,
            y: height,
        };

        let grass_texture = asset_server.load("ground.png");

        let mut tile_storage = TileStorage::empty(tilemap_size);
        let tilemap_entity = commands.spawn_empty().id();

        for x in 0..width {
            for y in 0..height {
                if let Some(ground_type) = layer.get_tile_type_at(x as usize, y as usize) {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = commands
                        .spawn((TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(ground_type.into()),
                            tilemap_id: TilemapId(tilemap_entity),
                            ..Default::default()
                        },))
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        }

        let tile_size = TilemapTileSize {
            x: TILE_MAP_TILE_WIDTH,
            y: TILE_MAP_TILE_HEIGHT,
        };
        let grid_size = tile_size.into();
        let map_type = TilemapType::default();

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            map_type,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(grass_texture),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        });
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EdgeDirection {
    Top,
    OutsideTopRight,
    Right,
    OutsideBottomRight,
    Bottom,
    OutsideBottomLeft,
    Left,
    OutsideTopLeft,
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

impl From<GroundTileType> for u32 {
    fn from(val: GroundTileType) -> Self {
        val as u32
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
#[derive(Clone)]
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
            tiles,
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
            tiles,
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

#[derive(Resource)]
pub struct LayerStack {
    /// A stack of layers to be rendered, the lower ones pushed on first
    stack: Vec<Layer>,
}

impl LayerStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    // pub fn add_layer(&mut self, to_add: Layer) {
    //     self.stack.push(to_add);
    // }

    pub fn add_layers(&mut self, to_add: Vec<Layer>) {
        for layer in to_add {
            self.stack.push(layer);
        }
    }

    pub fn get_layer_at_level(&self, level: usize) -> Option<&Layer> {
        self.stack.get(level)
    }

    pub fn get_num_layers(&self) -> usize {
        self.stack.len()
    }
}

/// A builder object for ground layers.
/// Note that this should not be used for anything other than ground layers, since they are somewhat
/// special in that they have coverage throughout the entire map. The other layers may have gaps in
/// them, which should be built on a per-entity, rather than per-layer, basis.
///
/// Another important consideration is that in some cases, this will actually produce _two_
/// composited layers: the ground layer, and the ground "decoration" layer.
#[derive(Clone)]
pub struct GroundLayerBuilder {
    ground_layer: Layer,
    ground_decoration_layer: Layer,
    edge_map: HashMap<(GroundTileType, EdgeDirection), GroundTileType>,
}

impl GroundLayerBuilder {
    pub fn new(width: usize, height: usize, base_type: GroundTileType) -> Self {
        let ground_layer = Layer::new_from_base_tile(width, height, base_type);
        let ground_decoration_layer = Layer::new(width, height);
        GroundLayerBuilder {
            ground_layer: ground_layer,
            ground_decoration_layer: ground_decoration_layer,
            edge_map: HashMap::new(),
        }
    }

    pub fn add_alternative_for(
        &mut self,
        ground_type: GroundTileType,
        alternative_type: GroundTileType,
        percent: f64,
    ) -> Self {
        let map_size = self.ground_layer.get_size();
        let mut rng = thread_rng();
        for i in 0..map_size.0 {
            for j in 0..map_size.1 {
                if let Some(tile_type_at_position) = self.ground_layer.get_tile_type_at(i, j) {
                    if tile_type_at_position == ground_type {
                        let random_num = rng.gen_range(0.0..1.0);
                        if random_num < percent {
                            self.ground_layer
                                .set_tile_type_at(i, j, alternative_type.clone());
                        }
                    }
                } else {
                    panic!(
                        "Cannot add an alternative for tile because there is nothing at ({}, {})",
                        i, j
                    );
                }
            }
        }

        self.clone()
    }

    pub fn add_edges_and_corner_tiles_for_base_tile(
        &mut self,
        base_tile: GroundTileType,
        edge_tiles: Vec<(EdgeDirection, GroundTileType)>,
    ) -> Self {
        for edge_tile_pair in edge_tiles {
            self.add_edge_type_for_tile(base_tile.clone(), edge_tile_pair.0, edge_tile_pair.1);
        }

        self.clone()
    }

    pub fn add_edge_type_for_tile(
        &mut self,
        base_tile: GroundTileType,
        direction: EdgeDirection,
        edge_tile_type: GroundTileType,
    ) -> Self {
        self.edge_map
            .insert((base_tile.clone(), direction), edge_tile_type);

        if let Some(default_tile) = self.ground_layer.get_default_tile_type() {
            if base_tile == default_tile {
                self.adjust_tiles_for_edges_of_map()
            }
        }

        self.clone()
    }

    fn adjust_tiles_for_edges_of_map(&mut self) {
        self.replace_tiles_along_edge(EdgeDirection::Top);
        self.replace_tiles_along_edge(EdgeDirection::Bottom);
        self.replace_tiles_along_edge(EdgeDirection::Left);
        self.replace_tiles_along_edge(EdgeDirection::Right);

        self.replace_tiles_in_corners_of_map();
    }

    // fn decorate_edge_tiles_differing_from_base(
    //     &mut self,
    //     base_tile: GroundTileType,
    //     replacement_tiles: (
    //         // Top
    //         GroundTileType,
    //         // Right
    //         GroundTileType,
    //         // Bottom
    //         GroundTileType,
    //         // Left
    //         GroundTileType,
    //     ),
    // ) -> Self {
    //     let (width, height) = self.ground_layer.get_size();
    //     for i in 1..width - 1 {
    //         for j in 1..height - 1 {
    //             // If i, j is our base tile and the tile below us (i, j-1) is something else
    //             // then add a decorator (bottom) to the top of the tile below us.
    //             if let Some(current_tile) = self.ground_layer.get_tile_type_at(i, j) {
    //                 if let Some(tile_below) = self.ground_layer.get_tile_type_at(i, j - 1) {
    //                     if current_tile == base_tile && current_tile != tile_below {
    //                         // info!("Setting ground decoration layer for bottom tile!");
    //                         self.ground_decoration_layer.set_tile_type_at(
    //                             i,
    //                             j - 1,
    //                             replacement_tiles.2.clone(),
    //                         );
    //                     }
    //                 }

    //                 if let Some(tile_right) = self.ground_layer.get_tile_type_at(i + 1, j) {
    //                     if current_tile == base_tile && current_tile != tile_right {
    //                         self.ground_decoration_layer.set_tile_type_at(
    //                             i + 1,
    //                             j,
    //                             replacement_tiles.1.clone(),
    //                         );
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     self.clone()
    // }

    fn replace_tiles_along_edge(&mut self, direction: EdgeDirection) {
        let map_size = self.ground_layer.get_size();
        if let Some(default_tile) = self.ground_layer.get_default_tile_type() {
            if let Some(replacement_tile) = self.edge_map.get(&(default_tile, direction.clone())) {
                if direction == EdgeDirection::Top || direction == EdgeDirection::Bottom {
                    let mut y_value = 0;
                    if direction == EdgeDirection::Top {
                        y_value = map_size.1 - 1
                    }

                    for i in 0..map_size.0 {
                        self.ground_layer
                            .set_tile_type_at(i, y_value, replacement_tile.clone());
                    }
                } else if direction == EdgeDirection::Left || direction == EdgeDirection::Right {
                    let mut x_value = 0;
                    if direction == EdgeDirection::Right {
                        x_value = map_size.1 - 1
                    }

                    for j in 0..map_size.1 {
                        self.ground_layer
                            .set_tile_type_at(x_value, j, replacement_tile.clone());
                    }
                }
            }
        }
    }

    fn replace_tiles_in_corners_of_map(&mut self) {
        let map_size = self.ground_layer.get_size();
        if let Some(base_tile_type) = self.ground_layer.get_default_tile_type() {
            let top_left = self
                .edge_map
                .get(&(base_tile_type.clone(), EdgeDirection::OutsideTopLeft));
            if top_left.is_some() {
                let tile_type = top_left.unwrap();
                self.ground_layer
                    .set_tile_type_at(0, map_size.1 - 1, tile_type.clone());
            }

            let top_right = self
                .edge_map
                .get(&(base_tile_type.clone(), EdgeDirection::OutsideTopRight));
            if top_right.is_some() {
                let tile_type = top_right.unwrap();
                self.ground_layer.set_tile_type_at(
                    map_size.0 - 1,
                    map_size.1 - 1,
                    tile_type.clone(),
                );
            }

            let bottom_left = self
                .edge_map
                .get(&(base_tile_type.clone(), EdgeDirection::OutsideBottomLeft));
            if bottom_left.is_some() {
                let tile_type = bottom_left.unwrap();
                self.ground_layer.set_tile_type_at(0, 0, tile_type.clone());
            }

            let bottom_right = self
                .edge_map
                .get(&(base_tile_type.clone(), EdgeDirection::OutsideBottomRight));
            if bottom_right.is_some() {
                let tile_type = bottom_right.unwrap();
                self.ground_layer
                    .set_tile_type_at(map_size.0 - 1, 0, tile_type.clone());
            }
        }
    }

    /// Generate another set of base tiles, on top of the initial one, based on noise generation
    /// and a cellular automata model.
    pub fn generate_variegated_base(&mut self, tile_type: GroundTileType) -> Self {
        let map_size = self.ground_layer.get_size();

        let map = MapgenMapBuilder::new(map_size.0, map_size.1)
            .with(DrunkardsWalk::new(
                mapgen::drunkard::DrunkSpawnMode::Random,
                22,
                0.25,
                1,
                mapgen::Symmetry::None,
            ))
            // .with(NoiseGenerator::new(0.665))
            // .with(CellularAutomata::new())
            // .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
            .build();

        for i in 0..map_size.0 {
            for j in 0..map_size.1 {
                let single_index = (j * map_size.0) + i;
                if map.tiles[single_index as usize] == TileType::Floor {
                    self.ground_layer.set_tile_type_at(i, j, tile_type.clone());
                }
            }
        }

        self.clone()
    }

    /// Replace an inside top-left corner of one tile type with another tile type.
    ///
    /// An "inside top left" corner is defined as a tile at (x, y) with a defined type where the
    /// next right-most tile (x+1, y) is of the same type, as is the tile just beneath it (x, y-1),
    /// but where the tile diagonally down and to the right (x+1, y-1) is of a different type.
    ///
    /// NOTE: This algorithm is correct, BUT we actually want to draw ON TOP OF the existing tile in
    ///       that's diagonally adjacent.
    // pub fn replace_inside_top_left_corners(
    //     &mut self,
    //     base_tile_type: GroundTileType,
    //     inside_corner_type: GroundTileType,
    // ) -> Self {
    //     let map_size = self.ground_layer.get_size();

    //     for i in 0..map_size.0 - 1 {
    //         for j in 0..map_size.1 - 1 {
    //             if self.ground_layer.get_tile_type_at(i, j).unwrap() != base_tile_type {
    //                 continue;
    //             }

    //             let next_right_tile = self.ground_layer.get_tile_type_at(i + 1, j);
    //             let next_bottom_tile = self.ground_layer.get_tile_type_at(i, j + 1);
    //             let next_diagonally_bottom_right_tile =
    //                 self.ground_layer.get_tile_type_at(i + 1, j - 1);

    //             // Bail if any of them are none.
    //             if next_right_tile.is_none()
    //                 || next_bottom_tile.is_none()
    //                 || next_diagonally_bottom_right_tile.is_none()
    //             {
    //                 continue;
    //             }

    //             if next_right_tile.unwrap() != base_tile_type
    //                 && next_bottom_tile.unwrap() == base_tile_type
    //                 && next_diagonally_bottom_right_tile.unwrap() != base_tile_type
    //             {
    //                 // info!("Adding inside corner to ground decoration layer!");
    //                 self.ground_decoration_layer.set_tile_type_at(
    //                     i + 1,
    //                     j - 1,
    //                     inside_corner_type.clone(),
    //                 );
    //             }
    //         }
    //     }

    //     self.clone()
    // }

    pub fn build(&self) -> Vec<Layer> {
        vec![
            self.ground_layer.clone(),
            self.ground_decoration_layer.clone(),
        ]
    }
}

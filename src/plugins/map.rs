use std::collections::HashMap;

use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    prelude::{Camera2dBundle, Commands, Component, Res},
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use rand::{thread_rng, Rng};

use mapgen::MapBuilder as MapgenMapBuilder;
use mapgen::{
    filter::CellularAutomata, AreaStartingPosition, NoiseGenerator, TileType, XStart, YStart,
};

use super::layer::{EdgeDirection, GroundTileType, Layer};

pub struct SettleTownMapPlugin;

impl Plugin for SettleTownMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TilemapPlugin);
        app.add_systems(Startup, setup_map);
    }
}

#[derive(Component)]
pub struct OverheadMapCamera;

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, ground: Res<Layer>) {
    commands.spawn((Camera2dBundle::default(), OverheadMapCamera));

    let map_size = ground.get_size();
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
            let ground_type = ground.get_tile_type_at(x as usize, y as usize).unwrap();
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

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
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

#[derive(Clone)]
pub struct MapBuilder {
    ground_layer: Layer,
    edge_map: HashMap<(GroundTileType, EdgeDirection), GroundTileType>,
}

impl MapBuilder {
    pub fn new(width: usize, height: usize, base_type: GroundTileType) -> Self {
        let map = Layer::new_from_base_tile(width, height, base_type);
        MapBuilder {
            ground_layer: map,
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

    /// Generate another set of base tiles, on top of the initial one, based on noise generation
    /// and a cellular automata model.
    pub fn generate_variegated_base(&mut self, tile_type: GroundTileType) -> Self {
        let map_size = self.ground_layer.get_size();

        let map = MapgenMapBuilder::new(map_size.0, map_size.1)
            .with(NoiseGenerator::new(0.70))
            .with(CellularAutomata::new())
            .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
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
    //     let map_size = self.ground_layer.get_map_size();

    //     for i in 0..map_size.x - 1 {
    //         for j in 0..map_size.y - 1 {
    //             if self
    //                 .ground_layer
    //                 .get_ground_type_at(i as usize, j as usize)
    //                 .unwrap()
    //                 != base_tile_type
    //             {
    //                 continue;
    //             }

    //             let next_right_tile = self
    //                 .ground_layer
    //                 .get_ground_type_at((i + 1) as usize, j as usize);
    //             let next_bottom_tile = self
    //                 .ground_layer
    //                 .get_ground_type_at(i as usize, (j + 1) as usize);
    //             let next_diagonally_bottom_right_tile = self
    //                 .ground_layer
    //                 .get_ground_type_at((i + 1) as usize, (j - 1) as usize);

    //             // Bail if any of them are none.
    //             if next_right_tile.is_none()
    //                 || next_bottom_tile.is_none()
    //                 || next_diagonally_bottom_right_tile.is_none()
    //             {
    //                 continue;
    //             }

    //             if next_right_tile.unwrap() == base_tile_type
    //                 && next_bottom_tile.unwrap() == base_tile_type
    //                 && next_diagonally_bottom_right_tile.unwrap() != base_tile_type
    //             {
    //                 self.ground_layer.set_ground_type_at(
    //                     i as usize,
    //                     j as usize,
    //                     inside_corner_type.clone(),
    //                 );
    //             }
    //         }
    //     }

    //     self.clone()
    // }

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

    pub fn build(&self) -> Vec<Layer> {
        vec![self.ground_layer.clone()]
    }
}

use std::collections::HashMap;

use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    prelude::{Camera2dBundle, Commands, Component, IntoSystem, Res},
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use rand::{thread_rng, Rng};

use crate::resources::ground::{EdgeDirection, GroundTileType, Map};

pub struct SettleTownMapPlugin;

impl Plugin for SettleTownMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TilemapPlugin);
        app.add_systems(Startup, setup_map);
    }
}

#[derive(Default, Component)]
struct LastUpdate {
    value: f64,
}

#[derive(Component)]
pub struct OverheadMapCamera;

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, ground: Res<Map>) {
    commands.spawn((Camera2dBundle::default(), OverheadMapCamera));

    let map_size = ground.get_map_size();
    let width = map_size.x as u32;
    let height = map_size.y as u32;

    let grass_texture = asset_server.load("dummy-lpc/tiles/grass.png");

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..width {
        for y in 0..height {
            let ground_type = ground.get_ground_type_at(x as usize, y as usize).unwrap();
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(ground_type.into()),
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                    LastUpdate::default(),
                ))
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
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(grass_texture),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

#[derive(Clone)]
pub struct MapBuilder {
    map_in_progress: Map,
    edge_map: HashMap<(GroundTileType, EdgeDirection), GroundTileType>,
}

impl MapBuilder {
    pub fn new(width: u32, height: u32, base_type: GroundTileType) -> Self {
        let map = Map::new(width, height, base_type);
        MapBuilder {
            map_in_progress: map,
            edge_map: HashMap::new(),
        }
    }

    pub fn add_alternative_for(
        &mut self,
        ground_type: GroundTileType,
        alternative_type: GroundTileType,
        percent: f64,
    ) -> Self {
        let map_size = self.map_in_progress.get_map_size();
        let mut rng = thread_rng();
        for i in 0..map_size.x {
            for j in 0..map_size.y {
                match self
                    .map_in_progress
                    .get_ground_type_at(i as usize, j as usize)
                {
                    None => panic!(
                        "Cannot build a map because there is nothing at ({}, {})",
                        i, j
                    ),
                    Some(g) => {
                        if g == ground_type {
                            let random_num = rng.gen_range(0.0..1.0);
                            if random_num < percent {
                                self.map_in_progress.set_ground_type_at(
                                    i as usize,
                                    j as usize,
                                    alternative_type.clone(),
                                );
                            }
                        }
                    }
                }
            }
        }

        Self {
            map_in_progress: self.map_in_progress.clone(),
            edge_map: self.edge_map.clone(),
        }
    }

    // pub fn add_row_edge_tile(&mut self, row: u32, tile: GroundTileType) -> Self {
    //     let map_size = self.map_in_progress.get_map_size();
    //     for i in 0..map_size.x {
    //         self.map_in_progress
    //             .set_ground_type_at(i as usize, row as usize, tile.clone());
    //     }

    //     Self {
    //         map_in_progress: self.map_in_progress.clone(),
    //         edge_map: self.edge_map.clone(),
    //     }
    // }

    // pub fn add_column_edge_tile(&mut self, column: u32, tile: GroundTileType) -> Self {
    //     let map_size = self.map_in_progress.get_map_size();
    //     for j in 0..map_size.y {
    //         self.map_in_progress
    //             .set_ground_type_at(column as usize, j as usize, tile.clone());
    //     }

    //     Self {
    //         map_in_progress: self.map_in_progress.clone(),
    //         edge_map: self.edge_map.clone(),
    //     }
    // }

    pub fn add_edge_type_for_tile(
        &mut self,
        base_tile: GroundTileType,
        direction: EdgeDirection,
        edge_tile_type: GroundTileType,
    ) -> Self {
        self.edge_map
            .insert((base_tile.clone(), direction), edge_tile_type);
        if base_tile == self.map_in_progress.get_default_ground_tile() {
            self.adjust_tiles_for_edges_of_map()
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
        let replacement_tile = self.edge_map.get(&(
            self.map_in_progress.get_default_ground_tile(),
            direction.clone(),
        ));

        let map_size = self.map_in_progress.get_map_size();
        if replacement_tile.is_some() {
            if direction == EdgeDirection::Top || direction == EdgeDirection::Bottom {
                let mut y_value = 0;
                if direction == EdgeDirection::Top {
                    y_value = map_size.y - 1
                }

                for i in 0..map_size.x {
                    self.map_in_progress.set_ground_type_at(
                        i as usize,
                        y_value as usize,
                        replacement_tile.unwrap().clone(),
                    );
                }
            } else if direction == EdgeDirection::Left || direction == EdgeDirection::Right {
                let mut x_value = 0;
                if direction == EdgeDirection::Right {
                    x_value = map_size.y - 1
                }

                for j in 0..map_size.y {
                    self.map_in_progress.set_ground_type_at(
                        x_value as usize,
                        j as usize,
                        replacement_tile.unwrap().clone(),
                    );
                }
            }
        }
    }

    fn replace_tiles_in_corners_of_map(&mut self) {
        let map_size = self.map_in_progress.get_map_size();

        let direction_top = self.edge_map.get(&(
            self.map_in_progress.get_default_ground_tile(),
            EdgeDirection::Top,
        ));

        let direction_bottom = self.edge_map.get(&(
            self.map_in_progress.get_default_ground_tile(),
            EdgeDirection::Bottom,
        ));

        let direction_left = self.edge_map.get(&(
            self.map_in_progress.get_default_ground_tile(),
            EdgeDirection::Left,
        ));

        let direction_right = self.edge_map.get(&(
            self.map_in_progress.get_default_ground_tile(),
            EdgeDirection::Right,
        ));

        if direction_top.is_some() && direction_left.is_some() {
            self.map_in_progress.set_ground_type_at(
                0 as usize,
                (map_size.x - 1) as usize,
                GroundTileType::GrassEdgeTopLeft,
            );
        }

        if direction_top.is_some() && direction_right.is_some() {
            self.map_in_progress.set_ground_type_at(
                (map_size.x - 1) as usize,
                (map_size.x - 1) as usize,
                GroundTileType::GrassEdgeTopRight,
            );
        }

        if direction_bottom.is_some() && direction_left.is_some() {
            self.map_in_progress.set_ground_type_at(
                0 as usize,
                0 as usize,
                GroundTileType::GrassEdgeBottomLeft,
            );
        }

        if direction_bottom.is_some() && direction_right.is_some() {
            self.map_in_progress.set_ground_type_at(
                (map_size.x - 1) as usize,
                0 as usize,
                GroundTileType::GrassEdgeBottomRight,
            );
        }
    }

    pub fn build(&self) -> Map {
        self.map_in_progress.clone()
    }
}

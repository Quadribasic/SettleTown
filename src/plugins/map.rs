use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    prelude::{Camera2dBundle, Commands, Component, Res},
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};

use crate::resources::ground::Map;

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

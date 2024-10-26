use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use macroquad::{
    color::WHITE,
    math::{vec2, Rect},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use strum::*;
use strum_macros::EnumIter;

use crate::{
    assets::{AssetLoader, TextureAtlas, TextureManager},
    world::World,
};

// Tiles in the world are drawn at 32x32 px.
pub const TILE_SIZE_X: f32 = 32.0;
pub const TILE_SIZE_Y: f32 = 32.0;

const ASSETS_JSON_FILE: &'static str = "assets/assets.json";

#[derive(Clone, Debug, Deserialize, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub enum TileType {
    // Desert,
    Dirt,
    Grass,
    GrassAlternate1,
}

impl PartialEq<String> for TileType {
    fn eq(&self, other: &String) -> bool {
        let mut lhs_string = String::new();
        let lhs_string_result = fmt::write(&mut lhs_string, format_args!("{:?}", self));
        if lhs_string_result.is_err() {
            return false;
        }

        lhs_string = lhs_string.to_lowercase();
        lhs_string == other.to_lowercase()
    }
}

impl PartialEq<&str> for TileType {
    fn eq(&self, other: &&str) -> bool {
        self == &other.to_string()
    }
}

impl PartialEq<TileType> for String {
    fn eq(&self, other: &TileType) -> bool {
        other == self
    }
}

impl PartialEq<TileType> for &str {
    fn eq(&self, other: &TileType) -> bool {
        other == self
    }
}

// pub struct Layer {
//     pub base_texture: Texture2D,
// }

// impl Layer {
//     pub fn new(base: Option<Texture2D>) -> Self {
//         let base_texture = Texture2D::from_file_with_format(
//             include_bytes!("../assets/desert-tiles/plain-desert-tile-1.png"),
//             None,
//         );

//         if base.is_none() {
//             Self { base_texture }
//         } else {
//             Self {
//                 base_texture: base.unwrap(),
//             }
//         }
//     }
// }

/// The renderer is the entry point to all drawing operations. It controls the state of the world
/// and displays this state within the window that's set up by the main application.
pub struct Renderer {
    game_world: World,

    // A set of layers to render, from lowest to highest
    // pub layers: Vec<Layer>,
    /// The default texture to show, if something happens that causes an error.
    pub default_texture: Texture2D,

    texture_manager: TextureManager,
    texture_atlas: Option<TextureAtlas>,
    renderable_tiles: HashMap<TileType, RenderableTile>,
}

impl Renderer {
    pub async fn new(window_width: f32, window_height: f32, map_file: Option<String>) -> Self {
        let asset_loader = AssetLoader::new(ASSETS_JSON_FILE);
        let texture_atlas = match asset_loader.get_texture_atlas() {
            Some(x) => Some(x),
            None => {
                println!("Error: Unable to load texture atlas from file {:?}. All textures from this asset file will be the default texture!", ASSETS_JSON_FILE);
                None
            }
        };

        let mut instance = Renderer {
            default_texture: Texture2D::from_file_with_format(
                include_bytes!("../assets/default-texture.png"),
                None,
            ),
            game_world: World::new_from_map_file(window_width, window_height, map_file),
            // layers: vec![Layer::new(None)],
            texture_atlas,
            renderable_tiles: HashMap::new(),
            texture_manager: TextureManager::new(),
        };

        instance.build_renderable_tile_set().await;

        instance
    }

    async fn build_renderable_tile_set(&mut self) {
        for tile_type in TileType::iter() {
            let renderable_tile = self.get_renderable_tile_for(&tile_type).await;
            self.renderable_tiles.insert(tile_type, renderable_tile);
        }
    }

    pub async fn get_renderable_tile_for(&mut self, tile_type: &TileType) -> RenderableTile {
        if self.texture_atlas.is_none() {
            let default_texture = self.default_texture.clone();
            return RenderableTile::new(0, 0, 32, 32, default_texture).await;
        } else {
            let tile_description_option = self
                .texture_atlas
                .as_ref()
                .unwrap()
                .clone()
                .get_tile_description_for_tile_type(tile_type);
            if tile_description_option.is_none() {
                let default_texture = self.default_texture.clone();
                return RenderableTile::new(0, 0, 32, 32, default_texture).await;
            } else {
                let tile_description = tile_description_option.unwrap();
                let tile_start = tile_description.get_start_location();

                let tile_size = tile_description.cell_size;
                let texture_file = self
                    .texture_manager
                    .get_texture(&tile_description.file)
                    .await;
                return RenderableTile::new(
                    tile_start.0,
                    tile_start.1,
                    tile_size.width,
                    tile_size.height,
                    texture_file,
                )
                .await;
            }
        }
    }

    pub async fn draw_world(&mut self) {
        let mut game_world = self.game_world.clone();
        game_world.draw(self).await;
        self.game_world = game_world;
    }
}

#[derive(Clone, Debug)]
pub struct RenderableTile {
    source_x: u32,
    source_y: u32,
    width: u32,
    height: u32,
    texture: Texture2D,
}

impl RenderableTile {
    pub async fn new(
        tile_start_x: u32,
        tile_start_y: u32,
        width: u32,
        height: u32,
        texture_file: Texture2D,
    ) -> Self {
        Self {
            source_x: tile_start_x,
            source_y: tile_start_y,
            width: width,
            height: height,
            texture: texture_file,
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(TILE_SIZE_X, TILE_SIZE_Y)),
                source: Some(Rect {
                    x: self.source_x as f32,
                    y: self.source_y as f32,
                    w: self.width as f32,
                    h: self.height as f32,
                }),
                ..Default::default()
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_show_grass_as_equal_to_tile_type_grass() {
        assert_eq!("Grass", TileType::Grass);
    }

    #[test]
    fn it_should_show_desert_as_not_equal_to_tile_type_grass() {
        assert_ne!("Desert", TileType::Grass);
    }
}

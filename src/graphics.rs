use macroquad::{
    color::WHITE,
    math::{vec2, Rect},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::world::World;

// Tiles in the world are 32x32 px.
pub const TILE_SIZE_X: f32 = 32.0;
pub const TILE_SIZE_Y: f32 = 32.0;

pub struct Layer {
    pub base_texture: Texture2D,
}

impl Layer {
    pub fn new(base: Option<Texture2D>) -> Self {
        let base_texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/desert-tiles/plain-desert-tile-1.png"),
            None,
        );

        if base.is_none() {
            Self { base_texture }
        } else {
            Self {
                base_texture: base.unwrap(),
            }
        }
    }

    // pub fn draw(&self) {
    //     let texture_width = 64.;
    //     let texture_height = 64.;

    //     for i in 0..19 {
    //         for j in 0..19 {
    //             draw_texture_ex(
    //                 &self.base_texture,
    //                 (i as f32) * texture_width,
    //                 (j as f32) * texture_height,
    //                 WHITE,
    //                 DrawTextureParams {
    //                     dest_size: Some(vec2(texture_width, texture_height)),
    //                     ..Default::default()
    //                 },
    //             );
    //         }
    //     }
    // }
}

pub struct Renderer {
    // A set of layers to render, from lowest to highest
    pub layers: Vec<Layer>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            layers: vec![Layer::new(None)],
        }
    }

    // pub fn draw(&self) {
    //     self.layers.iter().for_each(|l| l.draw());
    // }

    pub fn draw_world(&self, world: &mut World) {
        let world_size = world.get_world_size_in_tiles();
        for i in 0..world_size.0 {
            for j in 0..world_size.1 {
                let tile_type = world.get_tile_type_at(i, j).unwrap();
                let tile_texture = world.get_texture_for(tile_type);
                draw_texture_ex(
                    &tile_texture,
                    (i as f32) * TILE_SIZE_X,
                    (j as f32) * TILE_SIZE_Y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(TILE_SIZE_X, TILE_SIZE_Y)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

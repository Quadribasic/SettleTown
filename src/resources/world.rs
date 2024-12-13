use bevy::{
    math::{Rect, Vec2},
    prelude::Resource,
};

#[derive(Debug, Resource)]
pub struct WorldCoordinates {
    coordinate_rect: Rect,
}

impl WorldCoordinates {
    pub fn new(
        tile_size_x: f32,
        tile_size_y: f32,
        num_tiles_wide: usize,
        num_tiles_high: usize,
    ) -> Self {
        let radius_width_min: f32 = (tile_size_x * (num_tiles_wide as f32)) / 2.0;
        let radius_height_min: f32 = (tile_size_y * (num_tiles_high as f32)) / 2.0;
        let radius_width_max: f32 = (tile_size_x * (num_tiles_wide as f32)) / 2.0;
        let radius_height_max: f32 = (tile_size_y * (num_tiles_high as f32)) / 2.0;
        let coord_rect = Rect {
            min: Vec2::new(-radius_width_min, -radius_height_min),
            max: Vec2::new(radius_width_max, radius_height_max),
        };

        Self {
            coordinate_rect: coord_rect,
        }
    }

    pub fn get_min_x(&self) -> f32 {
        self.coordinate_rect.min.x
    }

    pub fn get_max_x(&self) -> f32 {
        self.coordinate_rect.max.x
    }

    pub fn get_min_y(&self) -> f32 {
        self.coordinate_rect.min.y
    }

    pub fn get_max_y(&self) -> f32 {
        self.coordinate_rect.max.y
    }

    // pub fn get_tile_indices_for_point(&self, p: Vec2) -> (usize, usize) {}
}

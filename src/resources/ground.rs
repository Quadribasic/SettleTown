use bevy::prelude::Resource;
use bevy_ecs_tilemap::map::TilemapSize;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Clone)]
pub enum GroundType {
    Grass = 10,
    GrassAlternate1 = 15,
    GrassAlternate2 = 16,
    GrassAlternate3 = 17,
}

impl Into<u32> for GroundType {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Resource)]
pub struct Map {
    map_size: TilemapSize,
    ground_tiles: Vec<Vec<GroundType>>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let mut tiles = vec![];

        // Each map tile is a grass tile, but 10% of all grass tiles should
        // be an alternate of some kind.
        let mut rng = thread_rng();
        for _i in 0..width {
            let mut row: Vec<GroundType> = vec![];
            for _j in 0..height {
                let mut ground_type = GroundType::Grass;
                let random_int: u32 = rng.gen_range(0..10);
                if random_int < 1 {
                    // Generate another random number to determine which alternate we want
                    let second_random_int: u32 = rng.gen_range(0..3);

                    ground_type = match second_random_int {
                        0 => GroundType::GrassAlternate1,
                        1 => GroundType::GrassAlternate2,
                        2 => GroundType::GrassAlternate3,
                        _ => GroundType::Grass,
                    };
                }

                row.push(ground_type);
            }

            tiles.push(row);
        }

        Map {
            map_size: TilemapSize {
                x: width,
                y: height,
            },
            ground_tiles: tiles,
        }
    }

    pub fn get_ground_type_at(&self, x: usize, y: usize) -> Option<GroundType> {
        match self.ground_tiles.get(x) {
            None => None,
            Some(val) => match val.get(y) {
                None => None,
                Some(ret_val) => Some(ret_val.clone()),
            },
        }
    }

    pub fn get_map_size(&self) -> TilemapSize {
        self.map_size.clone()
    }
}

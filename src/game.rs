// use crate::{graphics::Renderer, world::World};

// pub struct Engine {
//     world: World,
//     renderer: Renderer,
// }

// impl Engine {
//     pub fn new(window_width: f32, window_height: f32) -> Self {
//         let instance = Self {
//             world: World::new(&self.renderer, window_width, window_height),
//             renderer: Renderer::new(),
//         };

//         // Add our "ground" layer
//         // let ground_layer = instance.world.get_ground_layer();
//         // instance.renderer.add_layer();

//         instance
//     }

//     pub fn draw_map(&mut self) {
//         self.renderer.draw_world(&mut self.world);
//     }
// }

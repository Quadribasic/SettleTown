// use bevy::{
//     input::ButtonInput,
//     log::info,
//     prelude::{KeyCode, Query, Res, ResMut, With},
//     render::primitives::Aabb,
// };

// use crate::{plugins::player_character::PlayerCharacter, resources::config::SettleTownConfig};

// // pub fn toggle_aabb_rendering(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut config: ResMut<SettleTownConfig>,
//     query_player_character: Query<&mut Aabb, With<PlayerCharacter>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::KeyB) {
//         config.show_bounding_boxes = !config.show_bounding_boxes;
//     }

//     if !query_player_character.is_empty() {
//         info!("No player character has a bounding box yet");
//     } //else {
//       //     let player_char_aabb = query_player_character.single();
//       //     if config.show_bounding_boxes {
//       //         info!("Showing bounding box: {:?}", player_char_aabb);
//       //     }
//       // }
// }

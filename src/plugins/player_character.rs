// use bevy::{
//     app::{Plugin, Startup},
//     asset::{AssetServer, Assets, Handle},
//     log::info,
//     math::{UVec2, Vec3},
//     prelude::{Commands, Component, Image, Res, ResMut, Transform},
//     sprite::{TextureAtlas, TextureAtlasLayout},
//     utils::default,
// };

use std::time::Duration;

use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Assets, Handle},
    log::info,
    math::{UVec2, Vec3},
    prelude::{Commands, Component, Image, Res, ResMut, Transform},
    sprite::{TextureAtlas, TextureAtlasLayout},
    time::{Timer, TimerMode},
    utils::default,
};

use crate::components::sprites::DirectionalSpriteBundle;

pub struct SettleTownPlayerCharacterPlugin;

impl Plugin for SettleTownPlayerCharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_character_entity);
    }
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct PlayerCharacter;

fn spawn_player_character_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Spawning player character entity");
    // Load the necessary texture from a file for the player character
    let walkcycle_image: Handle<Image> =
        asset_server.load("dummy-lpc/sprites/people/male_walkcycle.png");
    // the sprite sheet has 9 sprites arranged in four rows, and they are all 64x64 pixels
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 9, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_character_sprite = DirectionalSpriteBundle {
        transform: Transform::from_scale(Vec3::splat(1.0))
            .with_translation(Vec3::new(0.0, 0.0, 2.0)),
        texture: walkcycle_image,
        ..default()
    };
    commands.spawn((
        PlayerCharacter,
        player_character_sprite.clone(),
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: (player_character_sprite.facing as usize) * 9,
        },
    ));

    info!("Finished spawning player character entity");
}

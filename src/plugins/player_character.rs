use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Assets, Handle},
    log::info,
    math::{UVec2, Vec3},
    prelude::{Commands, Component, Image, Res, ResMut, Transform},
    sprite::{TextureAtlas, TextureAtlasLayout},
    utils::default,
};

use crate::{
    components::sprites::{DirectionalSprite, DirectionalSpriteBundle},
    plugins::animation::AnimationConfig,
};

pub const PLAYER_CHARACTER_WALK_FPS: u8 = 10;
pub const PLAYER_CHARACTER_RUN_FPS: u8 = 100;

pub struct SettleTownPlayerCharacterPlugin;

impl Plugin for SettleTownPlayerCharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_character_entity);
    }
}

#[derive(Component, Debug)]
pub struct PlayerCharacter;

fn spawn_player_character_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Spawning player character entity");
    // Load the necessary texture from a file for the player character
    let walkcycle_image: Handle<Image> =
        asset_server.load("dummy-lpc/sprites/people/female_walkcycle.png");
    // the sprite sheet has 9 sprites arranged in four rows, and they are all 64x64 pixels
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 9, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_character_sprite = DirectionalSpriteBundle {
        transform: Transform::from_scale(Vec3::splat(1.0))
            .with_translation(Vec3::new(0.0, 0.0, 1.0)),
        texture: walkcycle_image,
        ..default()
    };

    // There are 9 frames to each walkcycle animation.
    let pc_animation = AnimationConfig::new(
        (player_character_sprite.facing.clone() as usize) * 9,
        ((player_character_sprite.facing.clone() as usize) * 9) + 8,
        PLAYER_CHARACTER_WALK_FPS,
    );

    let bundle = (
        PlayerCharacter,
        player_character_sprite.clone(),
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: pc_animation.first_sprite_index,
        },
        pc_animation,
        DirectionalSprite,
    );

    info!("Finished spawning player character entity: {:?}", &bundle);
    commands.spawn(bundle);
}

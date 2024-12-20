use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Assets, Handle},
    log::info,
    math::{UVec2, Vec2, Vec3},
    prelude::{Commands, Component, Image, Res, ResMut, Transform},
    sprite::{TextureAtlas, TextureAtlasLayout},
};

use crate::{
    components::sprites::{
        Direction, DirectionalSprite, DirectionalSpriteBundle, NonAnimatedApparel,
    },
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

// impl PlayerCharacter {
//     pub fn new(hat_entity_path: &str) -> Self {
//         PlayerCharacter {
//             hat_apparel_entity_path: String::from(hat_entity_path),
//         }
//     }
// }

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
    let player_char_facing = Direction::default();
    // There are 9 frames to each walkcycle animation.
    let pc_animation = AnimationConfig::new(
        (player_char_facing.clone() as usize) * 9,
        ((player_char_facing.clone() as usize) * 9) + 8,
        PLAYER_CHARACTER_WALK_FPS,
    );
    let player_character_sprite = DirectionalSpriteBundle::new(
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: pc_animation.first_sprite_index,
        },
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, 1.0)),
        walkcycle_image,
        player_char_facing,
    );

    let bundle = (
        PlayerCharacter,
        player_character_sprite.clone(),
        pc_animation,
        DirectionalSprite,
    );

    let player_character_entity = commands.spawn(bundle).id();

    let texture_atlas_layout_hat = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 25),
        1,
        4,
        None,
        None,
    ));

    let hat_image: Handle<Image> = asset_server.load("custom/sprites/apparel/cowboy_hat.png");
    let hat_apparel = NonAnimatedApparel::new(
        player_character_entity,
        Vec2::new(0.0, 12.0),
        hat_image,
        texture_atlas_layout_hat,
    );
    hat_apparel.spawn(commands);
}

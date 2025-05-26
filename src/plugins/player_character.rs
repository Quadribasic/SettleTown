use bevy::{
    app::Plugin,
    asset::{AssetServer, Assets, Handle},
    gizmos::gizmos::Gizmos,
    image::Image,
    log::info,
    math::{UVec2, Vec3},
    prelude::{
        Color, Commands, Component, OnEnter, Query, Res, ResMut, Sprite, TextureAtlas,
        TextureAtlasLayout, Transform, Update, Vec2, With,
    },
};

use bevy_state::state::States;

use crate::{
    components::sprites::{Direction, DirectionalSprite, DirectionalSpriteBundle},
    plugins::animation::{AnimationConfig, AnimationUtils},
    resources::config::SettleTownConfig, AssetQueue, // resources::config::SettleTownConfig,
};

pub const PLAYER_CHARACTER_WALK_FPS: u8 = 10;
pub const PLAYER_CHARACTER_RUN_FPS: u8 = 100;

pub struct SettleTownPlayerCharacterPlugin<S: States> {
    pub enabled_state: S,
}

impl<S: States> Plugin for SettleTownPlayerCharacterPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(self.enabled_state.clone()),
            spawn_player_character_entity,
        );
        app.add_systems(Update, draw_player_character_aabb);
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
    images: Res<Assets<Image>>,
    asset_queue: Res<AssetQueue>,
) {
    info!("Spawning player character entity");
    // Load the necessary texture from a file for the player character. Because we loaded this
    // asset during the splash screen loading phase, it should be available in the asset server
    // at this point.
    // let walkcycle_image: Handle<Image> =
    //     asset_server.load("custom/sprites/people/BadlandsBartWalkcycle.png");

    // XXX_jwir3: The asset server should be handling this and loading the image, but it seems
    //            the asset path is somehow different when we perform the above load _again_.
    let walkcycle_image = asset_queue.paths_to_ids.get(
        "custom/sprites/people/BadlandsBartWalkcycle.png",
    ).cloned().expect("BadlandsBartWalkcycle.png should have been loaded during the splash screen phase");
    info!("Player character walkcycle image handle: {:?}", walkcycle_image);
    info!("Player character walkcycle image load state: {:?}", asset_server.get_load_state(&walkcycle_image));
    let num_frames = AnimationUtils::get_num_frames_for_animation(images, walkcycle_image.clone(), 64, 64);

    // the sprite sheet has 8 sprites arranged in four rows, and they are all 64x64 pixels
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), num_frames.0, num_frames.1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_char_facing = Direction::default();

    // There are 8 frames to each walkcycle animation, computed through AnimationUtils.
    let pc_animation = AnimationConfig::new(
        (player_char_facing.clone() * num_frames.0) as usize,
        ((player_char_facing.clone() * num_frames.0) + num_frames.0 - 1) as usize,
        PLAYER_CHARACTER_WALK_FPS,
        (num_frames.0 as usize, num_frames.1 as usize),
    );
    info!("Player character animation config: {:?}", pc_animation);
    let player_character_sprite = Sprite::from_atlas_image(
        walkcycle_image,
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: pc_animation.first_sprite_index,
        },
    );
    let player_character_transform =
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, 1.0));

    let pc_character_directional_bundle = DirectionalSpriteBundle::new(
        player_character_sprite,
        player_character_transform,
        player_char_facing,
    );

    let bundle = (
        PlayerCharacter,
        pc_character_directional_bundle.clone(),
        pc_animation,
        DirectionalSprite,
    );

    let _player_character_entity = commands.spawn(bundle).id();

    // let texture_atlas_layout_hat = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
    //     UVec2::new(64, 25),
    //     1,
    //     4,
    //     None,
    //     None,
    // ));

    // let hat_image: Handle<Image> = asset_server.load("custom/sprites/apparel/cowboy_hat.png");
    // let hat_apparel = NonAnimatedApparel::new(
    //     player_character_entity,
    //     Vec2::new(0.0, 12.0),
    //     hat_image,
    //     texture_atlas_layout_hat,
    // );
    // hat_apparel.spawn(commands);
}

fn draw_player_character_aabb(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<PlayerCharacter>>,
    config: Res<SettleTownConfig>,
) {
    // If we don't have AABBs enabled, then don't draw them.
    if !config.base_config.world.enable_aabbs {
        return;
    }

    for transform in query.iter() {
        // The player sprite is 64x64, centered at its translation
        let size = Vec2::new(64.0, 64.0);
        let min = transform.translation.truncate() - size / 2.0;
        // let max = transform.translation.truncate() + size / 2.0;
        gizmos.rect_2d(min + size / 2.0, size, Color::srgb(0.0, 1.0, 0.0));
    }
}

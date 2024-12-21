use bevy::{
    asset::{Assets, Handle},
    math::{Vec2, Vec3},
    prelude::{
        BuildChildren, Bundle, Commands, Component, Entity, GlobalTransform, Image,
        InheritedVisibility, Res, ResMut, Transform, ViewVisibility, Visibility,
    },
    sprite::{Sprite, TextureAtlas, TextureAtlasLayout},
};

#[derive(Clone, Component, Debug)]
pub enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3,
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        self as usize
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::South
    }
}

/// Specifies the direction an entity is facing (given that the entity _can_ face more than one
/// direction). This is designed for the player character and NPCs, but can be used for any sprites
/// that have multiple directions it can face.
#[derive(Bundle, Clone, Debug, Default)]
pub struct DirectionalSpriteBundle {
    /// Specifies the rendering properties of the sprite, such as color tint and flip.
    pub sprite: Sprite,
    /// The local transform of the sprite, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the sprite. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// A reference-counted handle to the image asset to be drawn.
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// The direction the sprite is facing
    pub facing: Direction,
    /// A texture atlas for looking up the sprite's representative visual textures.
    pub atlas: TextureAtlas,
}

impl DirectionalSpriteBundle {
    pub fn new(
        texture_atlas: TextureAtlas,
        transform: Transform,
        texture: Handle<Image>,
        facing: Direction,
    ) -> Self {
        Self {
            atlas: texture_atlas,
            transform: transform,
            texture: texture,
            facing: facing,
            ..Default::default()
        }
    }
}

/// Marker struct for any directional sprite.
#[derive(Component, Debug)]
pub struct DirectionalSprite;

/// A sprite that doesn't animate with the parent entity to which it is attached. Thus, there are
/// only four different frames to the texture atlas: the four cardinal directions.
pub type NonAnimatedApparelSpriteBundle = DirectionalSpriteBundle;

impl NonAnimatedApparelSpriteBundle {
    pub fn create(
        value: &NonAnimatedApparel,
        atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let mut index: usize = 0;
        if let Some(layout) = atlas_layouts.get(&value.texture_atlas_layout) {
            index = (value.facing.clone() as usize) * (layout.size.y as usize)
        }

        Self {
            sprite: Sprite::default(),
            facing: Direction::default(),
            texture: value.texture.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(
                value.offset.x,
                value.offset.y,
                2.0,
            )),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            atlas: TextureAtlas {
                layout: value.texture_atlas_layout.clone(),
                index: index,
            },
        }
    }
}

#[derive(Component)]
pub struct Apparel;

pub trait Spawnable {
    fn spawn(
        &self,
        mut commands: Commands,
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    );
}
#[derive(Clone, Debug)]
pub struct NonAnimatedApparel {
    /// The vertical and horizontal offset of the sprite from the center of the parent
    pub offset: Vec2,

    /// The image representing the sprite in the game
    pub texture: Handle<Image>,

    /// The `TextureAtlasLayout` that defines how the different images within the image file should
    /// be interpreted as a texture atlas.
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,

    /// The Entity for which this apparel is attached
    pub parent_id: Entity,

    /// The direction the parent entity is facing
    pub facing: Direction,
}

impl NonAnimatedApparel {
    pub fn new(
        parent: Entity,
        offset: Vec2,
        texture: Handle<Image>,
        texture_atlas_layout: Handle<TextureAtlasLayout>,
        direction: Direction,
    ) -> Self {
        NonAnimatedApparel {
            offset: offset,
            parent_id: parent,
            texture: texture,
            texture_atlas_layout: texture_atlas_layout,
            facing: direction,
        }
    }

    pub fn spawn(
        &self,
        mut commands: Commands,
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let hat_bundle = (
            Apparel,
            NonAnimatedApparelSpriteBundle::create(self, texture_atlas_layouts),
        );
        commands.spawn(hat_bundle).set_parent(self.parent_id);
    }
}

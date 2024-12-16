use bevy::{
    asset::Handle,
    prelude::{
        Bundle, Component, GlobalTransform, Image, InheritedVisibility, Transform, ViewVisibility,
        Visibility,
    },
    sprite::Sprite,
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
}

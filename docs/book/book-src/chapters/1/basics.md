# Basics

**SettleTown** uses [Bevy](bevyengine.org), an open-source game engine. Bevy utilizes a model called
_ECS_, which stands for _Entity Component System_. It is a development paradigm often associated
with game development. It has a number of basic concepts associated with it, but the three main
concepts are described in the title:

## Main Aspects
The main aspects of an ECS are _entities_, _components_, and _systems_, as the title suggests.

### Entities
__Entities__ are data structures that store zero or more __Components__. They are general-purpose
game-world objects. They must have a unique id, but other than that, they can be effectively empty
structs. For example, a `PlayerCharacter` entity may exist, representing the player character for
the game.

```
struct PlayerCharacter(u64);
```


### Components
__Components__ are basic Rust structs (typically, they can also be `enums` or zero-sized types) that
implement the `Component` trait. They are data structures used to store data for an __Entity__. In
abstract terms, they are definitions that an entity contains a particular aspect. For example, the
`PlayerCharacter` entity, from above may have a `Position` component, allowing it to exist somewhere
in the world space.

```
#[derive(Component)]
struct WorldPosition {
    x: f32,
    y: f32,
}
```

### Systems
A __System__ is an operation that acts on all entities having a given component. For example, with
the above `PlayerCharacter` and `WorldPosition` structs, a `Movement` system may exist that queries
the entities with a particular position and updates that position.

```
fn update_position_system(mut query: Query<&mut WorldPosition>) {
    for position in &query {
        position.x = position.x + 1;
        position.y = position.y + 1;
    }
}
```

## Second-Tier Aspects

In addition to entities, components, and systems, Bevy also has some second-tier traits that allow
for a bit of abstraction above the main aspects.

### Bundles

Bundles are collections of components that allow for easy reuse. Bundles, in many ways, are like a
template for creating entities. They group the components together so it's easy to create multiple
entities with the same components.

### Plugins

All additional code to Bevy is considered a plugin. Plugins are just code that is added to the app
builder. You can think of a plugin as a modular set of code that can be reused.

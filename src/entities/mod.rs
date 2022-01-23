//! Entities
//!
//! Entities are based loosely around game objects; They bundle common functionality used by a
//! variety of systems, such as names for nouns, labelling components, or otherwise.

use bevy::prelude::*;

pub mod spaceships;

#[derive(Component)]
pub struct Name(pub String);

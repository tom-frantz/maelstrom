use bevy::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Hash)]
pub enum SpaceshipSprite {
    Programmer1,
    Programmer2,
    Programmer3,
}

impl SpaceshipSprite {
    const fn asset_name(&self) -> &'static str {
        match self {
            SpaceshipSprite::Programmer1 => "programmer-1.png",
            SpaceshipSprite::Programmer2 => "programmer-2.png",
            SpaceshipSprite::Programmer3 => "programmer-3.png",
        }
    }

    fn iter_assets() -> SpaceshipSpriteAssetIterator {
        SpaceshipSpriteAssetIterator {
            iter: SpaceshipSprite::iter(),
        }
    }
}

struct SpaceshipSpriteAssetIterator {
    iter: SpaceshipSpriteIter,
}

impl Iterator for SpaceshipSpriteAssetIterator {
    type Item = (SpaceshipSprite, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| {
            let asset_name = i.asset_name();
            (i, asset_name)
        })
    }
}

#[derive(Debug)]
pub struct SpaceshipSpriteHandles {
    handles: HashMap<SpaceshipSprite, Handle<Image>>,
}

impl SpaceshipSpriteHandles {
    pub(super) fn new(asset_server: Res<AssetServer>) -> SpaceshipSpriteHandles {
        SpaceshipSpriteHandles {
            handles: SpaceshipSprite::iter_assets()
                .map(|(sprite, path)| (sprite, asset_server.load(path)))
                .collect(),
        }
    }

    pub fn get_handle(&self, handle: &SpaceshipSprite) -> Option<&Handle<Image>> {
        self.handles.get(handle)
    }
}

pub(super) fn create_sprite_handles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SpaceshipSpriteHandles::new(asset_server))
}

pub fn create_spaceship_sprite(
    x: f32,
    y: f32,
    sprite: &SpaceshipSprite,
    handles: &Res<SpaceshipSpriteHandles>,
) -> SpriteBundle {
    SpriteBundle {
        texture: handles.get_handle(sprite).unwrap().clone(),
        transform: {
            let mut transform = Transform::default();
            transform.translation.x = x;
            transform.translation.y = y;
            transform
        },
        ..Default::default()
    }
}

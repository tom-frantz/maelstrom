use crate::ui::sprites::SPRITE_SCALE;
use bevy::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone, Copy, Component)]
pub enum SpaceshipSprite {
    Battlestar,
    Raptor,
    Viper,

    Astral,
    Botanical,
    Cloud,
    Colonial,
    Olympic,

    Shuttle,
    Mining,

    Civilian01,
    Civilian02,
    Civilian03,
    Civilian04,

    LargeCivilian01,
    LargeCivilian02,

    Industrial01,
    Industrial02,
    Industrial03,
    Industrial04,
}

impl Default for SpaceshipSprite {
    fn default() -> Self {
        SpaceshipSprite::Battlestar
    }
}

impl SpaceshipSprite {
    fn asset_name(&self) -> String {
        let path = match self {
            SpaceshipSprite::Battlestar => "battlestar.png",
            SpaceshipSprite::Raptor => "raptor.png",
            SpaceshipSprite::Viper => "viper.png",

            SpaceshipSprite::Astral => "astral.png",
            SpaceshipSprite::Botanical => "botanical.png",
            SpaceshipSprite::Cloud => "cloud.png",
            SpaceshipSprite::Colonial => "colonial.png",
            SpaceshipSprite::Olympic => "olympic.png",

            SpaceshipSprite::Shuttle => "shuttle.png",
            SpaceshipSprite::Mining => "mining.png",

            SpaceshipSprite::Civilian01 => "civilian_01.png",
            SpaceshipSprite::Civilian02 => "civilian_02.png",
            SpaceshipSprite::Civilian03 => "civilian_03.png",
            SpaceshipSprite::Civilian04 => "civilian_04.png",

            SpaceshipSprite::LargeCivilian01 => "large_civilian_01.png",
            SpaceshipSprite::LargeCivilian02 => "large_civilian_02.png",

            SpaceshipSprite::Industrial01 => "industrial_01.png",
            SpaceshipSprite::Industrial02 => "industrial_02.png",
            SpaceshipSprite::Industrial03 => "industrial_03.png",
            SpaceshipSprite::Industrial04 => "industrial_04.png",
        };

        return format!("spaceships/{}", path);
    }

    pub fn size(&self) -> Size {
        match self {
            SpaceshipSprite::Battlestar => Size::new(69.0, 27.0),
            SpaceshipSprite::Raptor => Size::new(3.0, 3.0),
            SpaceshipSprite::Viper => Size::new(2.0, 1.0),

            SpaceshipSprite::Astral => Size::new(44.0, 10.0),
            SpaceshipSprite::Botanical => Size::new(43.0, 10.0),
            SpaceshipSprite::Cloud => Size::new(28.0, 23.0),
            SpaceshipSprite::Colonial => Size::new(25.0, 9.0),
            SpaceshipSprite::Olympic => Size::new(48.0, 11.0),

            SpaceshipSprite::Shuttle => Size::new(4.0, 3.0),
            SpaceshipSprite::Mining => Size::new(29.0, 7.0),

            SpaceshipSprite::Civilian01 => Size::new(14.0, 11.0),
            SpaceshipSprite::Civilian02 => Size::new(11.0, 7.0),
            SpaceshipSprite::Civilian03 => Size::new(14.0, 4.0),
            SpaceshipSprite::Civilian04 => Size::new(9.0, 9.0),

            SpaceshipSprite::LargeCivilian01 => Size::new(36.0, 16.0),
            SpaceshipSprite::LargeCivilian02 => Size::new(40.0, 23.0),

            SpaceshipSprite::Industrial01 => Size::new(28.0, 7.0),
            SpaceshipSprite::Industrial02 => Size::new(29.0, 13.0),
            SpaceshipSprite::Industrial03 => Size::new(32.0, 7.0),
            SpaceshipSprite::Industrial04 => Size::new(17.0, 11.0),
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
    type Item = (SpaceshipSprite, String);

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
                .map(|(sprite, path)| (sprite, asset_server.load(&path)))
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
            transform.scale = Vec3::splat(SPRITE_SCALE);
            transform
        },
        ..Default::default()
    }
}

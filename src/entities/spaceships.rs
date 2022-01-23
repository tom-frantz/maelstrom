use crate::entities::Name;
use crate::ui::interactions::{Clickable, WorldClickEvent};
use crate::ui::sprites::spaceships::{
    create_spaceship_sprite, SpaceshipSprite, SpaceshipSpriteHandles,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Spaceship;

impl Spaceship {
    pub fn build() -> SpaceshipBuilder {
        Default::default()
    }
}

#[derive(Default, Clone)]
pub struct SpaceshipBuilder {
    x: f32,
    y: f32,
    name: String,
    sprite: SpaceshipSprite,
}

impl SpaceshipBuilder {
    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn sprite(&mut self, sprite: SpaceshipSprite) -> &mut Self {
        self.sprite = sprite;
        self
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn build(
        &mut self,
        commands: &mut Commands,
        handles: &Res<SpaceshipSpriteHandles>,
    ) -> Entity {
        let sprite_bundle = create_spaceship_sprite(self.x, self.y, &self.sprite, &handles);

        commands
            .spawn_bundle(sprite_bundle)
            .insert(Spaceship)
            .insert(Clickable::new(
                self.sprite.size(),
                WorldClickEvent::Spaceship,
            ))
            .insert(Name(self.name.clone()))
            // .insert(self.sprite)
            .id()
    }
}

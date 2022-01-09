use crate::ui::sprites::spaceships::{
    create_spaceship_sprite, SpaceshipSprite, SpaceshipSpriteHandles,
};
use crate::ui::utils::Clickable;
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

    pub fn build(
        &mut self,
        mut commands: &mut Commands,
        handles: &Res<SpaceshipSpriteHandles>,
    ) -> Entity {
        let sprite_bundle = create_spaceship_sprite(self.x, self.y, &self.sprite, &handles);
        commands
            .spawn_bundle(sprite_bundle)
            .insert(Spaceship)
            .insert(Clickable)
            .insert(self.sprite)
            .id()
    }
}

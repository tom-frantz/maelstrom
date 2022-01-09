use bevy::prelude::StartupStage::PostStartup;
use bevy::prelude::*;

use maelstrom::create_app;
use maelstrom::ui::sprites::spaceships::{
    create_spaceship_sprite, SpaceshipSprite, SpaceshipSpriteHandles,
};

/// interesting
fn create_spaceship_sprites(mut commands: Commands, sprite_handles: Res<SpaceshipSpriteHandles>) {
    let sprite = create_spaceship_sprite(0.0, 0.0, &SpaceshipSprite::Programmer1, &sprite_handles);
    commands.spawn_bundle(sprite);

    let sprite =
        create_spaceship_sprite(100.0, 0.0, &SpaceshipSprite::Programmer2, &sprite_handles);
    commands.spawn_bundle(sprite);

    let sprite =
        create_spaceship_sprite(200.0, 0.0, &SpaceshipSprite::Programmer3, &sprite_handles);
    commands.spawn_bundle(sprite);
}

fn main() {
    let mut app = create_app();
    app.add_startup_system_to_stage(PostStartup, create_spaceship_sprites);
    app.run();
}

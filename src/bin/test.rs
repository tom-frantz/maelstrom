use bevy::prelude::StartupStage::PostStartup;
use bevy::prelude::*;

use maelstrom::create_app;
use maelstrom::entities::spaceships::Spaceship;
use maelstrom::ui::sprites::spaceships::{SpaceshipSprite, SpaceshipSpriteHandles};

/// interesting
fn create_spaceship_sprites(mut commands: Commands, sprite_handles: Res<SpaceshipSpriteHandles>) {
    Spaceship::build()
        .sprite(SpaceshipSprite::Programmer1)
        .pos(0.0, 0.0)
        .name("Viper 892".to_string())
        .build(&mut commands, &sprite_handles);

    Spaceship::build()
        .sprite(SpaceshipSprite::Programmer2)
        .pos(50.0, 0.0)
        .name("Raptor 243".to_string())
        .build(&mut commands, &sprite_handles);
    Spaceship::build()
        .sprite(SpaceshipSprite::Programmer3)
        .pos(100.0, 0.0)
        .name("Galactica".to_string())
        .build(&mut commands, &sprite_handles);
}

fn main() {
    let mut app = create_app();
    app.add_startup_system_to_stage(PostStartup, create_spaceship_sprites);
    app.run();
}

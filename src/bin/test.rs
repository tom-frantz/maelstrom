use bevy::prelude::StartupStage::PostStartup;
use bevy::prelude::*;

use maelstrom::create_app;
use maelstrom::entities::spaceships::Spaceship;
use maelstrom::ui::sprites::spaceships::{SpaceshipSprite, SpaceshipSpriteHandles};

/// interesting
fn create_spaceship_sprites(mut commands: Commands, sprite_handles: Res<SpaceshipSpriteHandles>) {
    Spaceship::build()
        .sprite(SpaceshipSprite::Battlestar)
        .pos(0.0, 0.0)
        .name("Galactica".to_string())
        .build(&mut commands, &sprite_handles);

    Spaceship::build()
        .sprite(SpaceshipSprite::Raptor)
        .pos(50.0, 90.0)
        .name("Raptor 243".to_string())
        .build(&mut commands, &sprite_handles);
    Spaceship::build()
        .sprite(SpaceshipSprite::Viper)
        .pos(55.0, 110.0)
        .name("Viper Nugget".to_string())
        .build(&mut commands, &sprite_handles);
}

fn main() {
    let mut app = create_app();
    app.add_startup_system_to_stage(PostStartup, create_spaceship_sprites);
    app.run();
}

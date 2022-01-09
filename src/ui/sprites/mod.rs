use crate::ui::sprites::spaceships::create_sprite_handles;
use bevy::prelude::StartupStage::Startup;
use bevy::prelude::*;

pub mod spaceships;

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub(super) struct UiSpritesPlugin;
impl Plugin for UiSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_startup_system_to_stage(Startup, create_sprite_handles);
    }
}

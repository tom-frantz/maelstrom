use crate::ui::sprites::planets::{animate_sprite_system, planet_setup};
use crate::ui::sprites::spaceships::create_sprite_handles;
use bevy::prelude::StartupStage::Startup;
use bevy::prelude::*;

pub mod interactions;
pub mod planets;
pub mod spaceships;

pub(super) const SPRITE_SCALE: f32 = 5.0;

#[derive(Component)]
pub(super) struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

pub(super) struct UiSpritesPlugin;
impl Plugin for UiSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_startup_system_to_stage(Startup, create_sprite_handles)
            .add_startup_system(planet_setup)
            .add_system(animate_sprite_system);
    }
}

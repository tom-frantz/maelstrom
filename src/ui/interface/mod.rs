use crate::ui::interface::sidebar::SidebarPlugin;
use crate::ui::interface::text::FontResource;
use bevy::prelude::StartupStage::PreStartup;
use bevy::prelude::*;

mod resource;
mod text;
pub(self) mod ui_bundle;

pub mod sidebar;
pub mod timer;

fn spawn_bundle(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub(super) struct UiInterfacePlugin;
impl Plugin for UiInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontResource>()
            .add_startup_system_to_stage(PreStartup, spawn_bundle)
            .add_plugin(SidebarPlugin);
    }
}

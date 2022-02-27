use crate::ui::interface::sidebar::SidebarPlugin;
use crate::ui::interface::text::FontResource;
use bevy::prelude::StartupStage::PreStartup;
use bevy::prelude::*;

mod resource;
mod text;

pub mod sidebar;
pub mod timer;

use crate::ui::interface::resource::UiResource;
use crate::ui::interface::sidebar::{
    CloseButton, Sidebar, SidebarContainer, BORDER_COLOUR, SIDEBAR_BORDER_WIDTH, SIDEBAR_COLOUR,
    SIDEBAR_WINDOW_WIDTH,
};
use crate::GameState;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::AlignItems::FlexStart;
use bevy::prelude::FlexDirection::ColumnReverse;
use bevy::prelude::Val::{Percent, Px};
use bevy::prelude::*;

// A custom bundle that combines a marker and some Bevy UI bundle for ease
#[derive(Bundle)]
pub struct UiBundle<M, B>
where
    M: Component,
    B: Bundle,
{
    marker: M,

    #[bundle]
    ui_bundle: B,
}

impl<M, B> UiBundle<M, B>
where
    M: Component,
    B: Bundle,
{
    pub fn new(ui_bundle: B, marker: M) -> Self {
        UiBundle { marker, ui_bundle }
    }
    // pub fn get_sidebar_content_from_container(mut container: EntityCommands) -> EntityCommands {}
}

trait MarkedUi<B>
where
    Self: Component + Sized + Default,
    B: Bundle,
{
    fn get_bundle() -> B;

    fn create_ui() -> UiBundle<Self, B> {
        let bundle = Self::get_bundle();
        return UiBundle::new(bundle, Default::default());
    }

    fn spawn_bundle<'w, 's, 'a>(commands: &'a mut Commands<'w, 's>) -> EntityCommands<'w, 's, 'a> {
        commands.spawn_bundle(Self::create_ui())
    }

    // returns commands to the child.
    fn spawn_child_bundle<'w, 's, 'a>(
        parent: &'a mut ChildBuilder<'w, 's, 'a>,
    ) -> EntityCommands<'w, 's, 'a> {
        parent.spawn_bundle(Self::create_ui())
    }
}

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

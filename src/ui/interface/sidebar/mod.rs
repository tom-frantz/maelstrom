use crate::ui::interface::resource::UiResource;
use crate::ui::interface::sidebar::interaction::update_sidebar;
use crate::ui::interface::ui_bundle::UiBundle;
use crate::GameState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::AlignItems::FlexStart;
use bevy::prelude::FlexDirection::ColumnReverse;
use bevy::prelude::Val::{Percent, Px};
use bevy::prelude::*;

mod interaction;
mod spaceship;
mod state;

pub(super) const SIDEBAR_WINDOW_WIDTH: Val = Px(500.0);
pub(super) const SIDEBAR_BORDER_WIDTH: Val = Px(5.0);

pub(super) const BORDER_COLOUR: &'static str = "444";
pub(super) const SIDEBAR_COLOUR: &'static str = "222";

#[derive(Component)]
pub(super) struct Sidebar;

#[derive(Component)]
pub(super) struct SidebarContainer;
type SidebarQuery<'w, 's, 't0> = Query<'w, 's, (Entity, &'t0 mut Children), With<Sidebar>>;

#[derive(Component)]
pub(super) struct CloseButton;

fn handle_close_button(
    mut commands: Commands,
    q_button: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
    mut ui_resource: ResMut<UiResource>,
    mut game_state: ResMut<State<GameState>>,
) {
    let close_button = q_button.get_single();

    if let Ok(interaction) = close_button {
        match interaction {
            Interaction::Clicked => ui_resource.close_sidebar(&mut commands, game_state),
            _ => {}
        }
    }
}

fn create_sidebar(
    mut commands: Commands,
    mut ui_resource: ResMut<UiResource>,
    mut game_state: ResMut<State<GameState>>,
) {
    assert_eq!(game_state.current(), &GameState::InGame);
    UiBundle::create_empty_sidebar(&mut commands, &mut ui_resource, game_state);
}

pub(super) struct SidebarPlugin;
impl Plugin for SidebarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiResource>()
            .add_system(update_sidebar)
            .add_system(handle_close_button);
    }
}

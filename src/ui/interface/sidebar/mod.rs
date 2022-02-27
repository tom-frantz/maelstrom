use crate::ui::interface::resource::UiResource;
use crate::ui::interface::sidebar::interaction::update_sidebar;
use crate::ui::interface::{MarkedUi, UiBundle};
use crate::GameState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::AlignItems::FlexStart;
use bevy::prelude::Component;
use bevy::prelude::FlexDirection::ColumnReverse;
use bevy::prelude::Val::{Percent, Px};
use bevy::prelude::*;

mod interaction;
mod spaceship;
mod state;

pub(super) const SIDEBAR_WINDOW_WIDTH: Val = Px(1000.0);
pub(super) const SIDEBAR_BORDER_WIDTH: Val = Px(5.0);

pub(super) const BORDER_COLOUR: &'static str = "444";
pub(super) const SIDEBAR_COLOUR: &'static str = "222";

#[derive(Component, Default)]
pub(super) struct Sidebar;

#[derive(Component, Default)]
pub(super) struct SidebarContainer;
type SidebarQuery<'w, 's, 't0> = Query<'w, 's, (Entity, &'t0 mut Children), With<Sidebar>>;

#[derive(Component, Default)]
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

impl MarkedUi<NodeBundle> for Sidebar {
    fn get_bundle() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Percent(100.0), Percent(100.0)),
                flex_direction: ColumnReverse,
                align_items: FlexStart,
                ..Default::default()
            },
            color: UiColor(Color::hex(SIDEBAR_COLOUR).unwrap()),
            ..Default::default()
        }
    }
}

impl MarkedUi<NodeBundle> for SidebarContainer {
    fn get_bundle() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(SIDEBAR_WINDOW_WIDTH, Percent(100.0)),
                padding: Rect::all(SIDEBAR_BORDER_WIDTH),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::hex(BORDER_COLOUR).unwrap()),
            ..Default::default()
        }
    }
}

impl MarkedUi<ButtonBundle> for CloseButton {
    fn get_bundle() -> ButtonBundle {
        ButtonBundle {
            color: Color::RED.into(),
            style: Style {
                align_self: AlignSelf::FlexEnd,
                size: Size {
                    width: Px(10.0),
                    height: Px(10.0),
                },
                margin: Rect {
                    left: SIDEBAR_BORDER_WIDTH,
                    right: SIDEBAR_BORDER_WIDTH,
                    top: Px(0.0),
                    bottom: SIDEBAR_BORDER_WIDTH,
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub(super) struct SidebarPlugin;
impl Plugin for SidebarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiResource>()
            .add_system(update_sidebar)
            .add_system(handle_close_button);
    }
}

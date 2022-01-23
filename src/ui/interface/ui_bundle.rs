use crate::ui::interface::resource::UiResource;
use crate::ui::interface::sidebar::{
    CloseButton, Sidebar, SidebarContainer, BORDER_COLOUR, SIDEBAR_BORDER_WIDTH, SIDEBAR_COLOUR,
    SIDEBAR_WINDOW_WIDTH,
};
use crate::GameState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::AlignItems::FlexStart;
use bevy::prelude::FlexDirection::ColumnReverse;
use bevy::prelude::Val::{Percent, Px};
use bevy::prelude::*;

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

impl UiBundle<Sidebar, NodeBundle> {
    pub fn sidebar_bundle() -> UiBundle<Sidebar, NodeBundle> {
        UiBundle::new(
            NodeBundle {
                style: Style {
                    size: Size::new(Percent(100.0), Percent(100.0)),
                    flex_direction: ColumnReverse,
                    align_items: FlexStart,
                    ..Default::default()
                },
                color: UiColor(Color::hex(SIDEBAR_COLOUR).unwrap()),
                ..Default::default()
            },
            Sidebar,
        )
    }
}

impl UiBundle<SidebarContainer, NodeBundle> {
    pub fn sidebar_container_bundle() -> Self {
        UiBundle::new(
            NodeBundle {
                style: Style {
                    size: Size::new(SIDEBAR_WINDOW_WIDTH, Percent(100.0)),
                    padding: Rect::all(SIDEBAR_BORDER_WIDTH),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: UiColor(Color::hex(BORDER_COLOUR).unwrap()),
                ..Default::default()
            },
            SidebarContainer,
        )
    }

    pub fn create_empty_sidebar<'w, 's, 'a>(
        mut commands: &'a mut Commands<'w, 's>,
        mut ui_tracker: &mut UiResource,
        mut game_state: ResMut<State<GameState>>,
    ) -> EntityCommands<'w, 's, 'a> {
        // TODO spawn all components, return entity commands for SidebarContent
        let mut root = commands.spawn_bundle(UiBundle::sidebar_container_bundle());
        let mut content: Option<Entity> = None;
        root.with_children(|root| {
            root.spawn_bundle(UiBundle::close_button_bundle()).id();
            let content_entity = root.spawn_bundle(UiBundle::sidebar_bundle()).id();
            content = Some(content_entity);
        });

        ui_tracker.open_sidebar(root.id(), content.unwrap());
        game_state.push(GameState::SidebarOpen).unwrap();

        root
    }
}

impl UiBundle<CloseButton, ButtonBundle> {
    pub fn close_button_bundle() -> Self {
        UiBundle::new(
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
            },
            CloseButton,
        )
    }
}

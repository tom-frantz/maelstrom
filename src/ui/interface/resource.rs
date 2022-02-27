use crate::ui::interface::sidebar::{CloseButton, Sidebar, SidebarContainer};
use crate::ui::interface::{MarkedUi, UiBundle};
use crate::GameState;
use bevy::prelude::*;

#[derive(Default, Debug)]
pub(super) struct UiResource {
    container: Option<Entity>,
    content: Option<Entity>,
}

impl UiResource {
    pub fn open_sidebar(&mut self, container: Entity, content: Entity) {
        self.container = Some(container);
        self.content = Some(content);
    }

    pub fn sidebar_is_open(&self) -> bool {
        self.content.is_some()
    }

    pub fn get_or_create_sidebar_content(
        &mut self,
        commands: &mut Commands,
        mut game_state: ResMut<State<GameState>>,
    ) -> Entity {
        if let None = self.content {
            let mut root = SidebarContainer::spawn_bundle(commands);
            let mut content: Option<Entity> = None;

            root.with_children(|mut root| {
                root.spawn_bundle(CloseButton::get_bundle());
                let content_entity = root.spawn_bundle(Sidebar::get_bundle()).id();
                content = Some(content_entity);
            });

            self.open_sidebar(root.id(), content.unwrap());
            game_state.push(GameState::SidebarOpen).unwrap();
        }

        self.content.unwrap()
    }

    pub fn close_sidebar(
        &mut self,
        commands: &mut Commands,
        mut game_state: ResMut<State<GameState>>,
    ) {
        commands.entity(self.container.unwrap()).despawn_recursive();
        if let GameState::SidebarOpen = game_state.current() {
            game_state.pop();
        }

        self.container = None;
        self.content = None;
    }
}

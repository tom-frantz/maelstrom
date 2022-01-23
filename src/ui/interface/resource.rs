use crate::ui::interface::ui_bundle::UiBundle;
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
        game_state: ResMut<State<GameState>>,
    ) -> Entity {
        if let None = self.content {
            UiBundle::create_empty_sidebar(commands, self, game_state);
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

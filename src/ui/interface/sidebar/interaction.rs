use crate::ui::interactions::WorldClickEvent;
use crate::ui::interface::resource::UiResource;
use crate::ui::interface::sidebar::spaceship::{render_spaceship_info, SpaceshipQuery};
use crate::ui::interface::sidebar::SidebarQuery;
use crate::ui::interface::FontResource;
use crate::GameState;
use bevy::prelude::*;

pub(super) fn update_sidebar(
    mut events: EventReader<(WorldClickEvent, Entity)>,
    mut commands: Commands,
    q_spaceship: SpaceshipQuery,

    fonts: Res<FontResource>,
    mut ui: ResMut<UiResource>,
    mut game_state: ResMut<State<GameState>>,
) {
    if let Some(event) = events.iter().last() {
        // any event will trigger the sidebar opening if it's not already.
        let sidebar_content = ui.get_or_create_sidebar_content(&mut commands, game_state);
        let (event, entity): (WorldClickEvent, Entity) = *event;

        let mut sidebar_commands = commands.entity(sidebar_content);
        match event {
            WorldClickEvent::Spaceship => render_spaceship_info(
                q_spaceship.get(entity).unwrap(),
                &mut sidebar_commands,
                fonts,
            ),
        }
    }
}

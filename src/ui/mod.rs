use bevy::prelude::*;
use bevy::window::WindowMode::BorderlessFullscreen;

use crate::ui::interactions::{interaction_generator, WorldClickEvent};
use crate::ui::interface::UiInterfacePlugin;
use crate::ui::sprites::UiSpritesPlugin;

pub(crate) mod interactions;
pub mod interface;
pub mod sprites;

pub(super) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // This currently doesn't work, but will in the next update fingers crossed.
            .insert_resource(WindowDescriptor {
                title: "Maelstrom".to_string(),
                mode: BorderlessFullscreen,
                ..Default::default()
            })
            .add_plugin(UiSpritesPlugin)
            .add_plugin(UiInterfacePlugin)
            .add_event::<(WorldClickEvent, Entity)>()
            .add_system(interaction_generator)
            .add_system(|mut events: EventReader<(WorldClickEvent, Entity)>| {
                for event in events.iter() {
                    println!("WOW READING THE EVENT: {:?}", *event)
                }
            });
    }
}

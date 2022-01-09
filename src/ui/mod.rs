use crate::ui::sprites::UiSpritesPlugin;
use bevy::prelude::*;

pub mod interface;
pub mod sprites;

pub(super) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiSpritesPlugin);
    }
}

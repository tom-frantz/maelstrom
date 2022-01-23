//! Maelstrom: A game loosely inspired by Battlestar Galactica and Homeworld.
//!
//!

use crate::state::GameState;
use crate::ui::UiPlugin;
use bevy::prelude::*;

pub mod entities;
pub mod mechanics;
pub(crate) mod state;
pub mod ui;

/// This function is used to bootstrap all the core functionality of the game, including anything
/// that is necessary for the running of the game.
pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_state(GameState::InGame)
        .add_plugin(UiPlugin);
    app
}

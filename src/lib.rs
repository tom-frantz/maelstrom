//! Maelstrom: A game loosely inspired by Battlestar Galactica and Homeworld.
//!
//!

use crate::mechanics::MechanicsPlugin;
use crate::state::GameState;
use crate::ui::UiPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode::BorderlessFullscreen;

pub mod entities;
pub mod mechanics;
pub(crate) mod state;
pub mod ui;

/// This function is used to bootstrap all the core functionality of the game, including anything
/// that is necessary for the running of the game.
pub fn create_app() -> App {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Maelstrom".to_string(),
            // mode: BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::InGame)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(UiPlugin)
        .add_plugin(MechanicsPlugin);
    app
}

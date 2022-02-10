use crate::mechanics::game_time::GameTime;
use bevy::prelude::*;

pub mod game_time;

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTime>()
            .add_system(GameTime::update_time);
    }
}

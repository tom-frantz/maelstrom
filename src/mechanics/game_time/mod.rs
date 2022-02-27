use bevy::prelude::*;
use num::Integer;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum GameSpeed {
    Paused,
    Slow,
    Normal,
    Fast,
}

impl Default for GameSpeed {
    fn default() -> Self {
        GameSpeed::Paused
    }
}

#[derive(Default, Debug)]
pub struct GameTime {
    sub_tick: u8,
    tick: u64,
    speed: GameSpeed,
}

pub struct GameTimeRepresentation {
    hour: u8,
    day: u64,
}

impl GameTime {
    pub fn update_time(mut this: ResMut<Self>) {
        this.update();
    }

    fn update(&mut self) {
        if !(self.speed == GameSpeed::Paused) {
            self.sub_tick += 1;

            let max_step: u8 = match self.speed {
                GameSpeed::Slow => 5,
                GameSpeed::Normal => 3,
                GameSpeed::Fast => 2,
                GameSpeed::Paused => {
                    panic!("This shouldn't happen")
                }
            };

            if self.sub_tick >= max_step {
                self.sub_tick = 0;
                self.tick += 1;
            }
        }
    }

    pub fn get_time_representation(&mut self) -> GameTimeRepresentation {
        let (day, hour) = self.tick.div_rem(&24);
        GameTimeRepresentation {
            hour: hour as u8,
            day,
        }
    }
}

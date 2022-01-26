#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum GameSpeed {
    Paused,
    Slow,
    Normal,
    Fast,
}

pub struct GameTime {
    sub_tick: u8,
    tick: u64,
    speed: GameSpeed,
}

impl GameTime {
    fn test_system() {
        println!("DOUBLE OUCH")
    }

    fn update(&mut self) {
        if !(self.speed == GameSpeed::Paused) {
            self.sub_tick += 1;

            match self.speed {
                GameSpeed::Slow => {}
                GameSpeed::Normal => {}
                GameSpeed::Fast => {}
                GameSpeed::Paused => {
                    panic!("This shouldn't happen")
                }
            }
        }
    }
}

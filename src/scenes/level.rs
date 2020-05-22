use ggez::{event, Context, GameResult};

use super::scene;
use crate::scenes;

pub struct LevelScene {
    done: bool,
}

impl LevelScene {
    pub fn new(_ctx: &mut Context) -> Self {
        let done = false;

        LevelScene { done }
    }
}

impl scene::Scene<event::KeyCode> for LevelScene {
    fn update(&mut self, _ctx: &mut Context) -> scenes::Switch {
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, ev: ggez::event::KeyCode, _started: bool) {
        if ev == event::KeyCode::Escape {
            self.done = true;
        }
    }
}

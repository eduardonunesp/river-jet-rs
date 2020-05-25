use ggez::{event, Context, GameResult};

use super::scene;
use crate::scenes;

pub struct MenuScene {
  done: bool,
  quit: bool,
}

impl MenuScene {
  #[allow(dead_code)]
  pub fn new(_ctx: &mut Context) -> Self {
    let done = false;
    let quit = false;

    MenuScene { done, quit }
  }
}

impl scene::Scene<event::KeyCode> for MenuScene {
  fn update(&mut self, ctx: &mut Context) -> scenes::Switch {
    if self.quit {
      event::quit(ctx);
      return scene::SceneSwitch::None;
    }

    if self.done {
      let level = Box::new(scenes::level::LevelScene::new(ctx));
      self.done = false;
      scene::SceneSwitch::Push(level)
    } else {
      scene::SceneSwitch::None
    }
  }

  fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn name(&self) -> &str {
    "MenuScene"
  }

  fn input(&mut self, ev: ggez::event::KeyCode, started: bool) {
    if ev == event::KeyCode::Tab && !started {
      self.done = true;
    }

    if ev == event::KeyCode::Escape {
      self.quit = true;
    }
  }
}

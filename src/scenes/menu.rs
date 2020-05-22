use ggez::nalgebra as na;
use ggez::{event, graphics, Context, GameResult};

use super::scene;
use crate::scenes;

pub struct MenuScene {
  done: bool,
}

impl MenuScene {
  pub fn new(_ctx: &mut Context) -> Self {
    let done = false;

    MenuScene { done }
  }
}

impl scene::Scene<event::KeyCode> for MenuScene {
  fn update(&mut self, ctx: &mut Context) -> scenes::Switch {
    if self.done {
      let level = Box::new(scenes::level::LevelScene::new(ctx));
      self.done = false;
      scene::SceneSwitch::Push(level)
    } else {
      scene::SceneSwitch::None
    }
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      na::Point2::new(0.0, 0.0),
      100.0,
      2.0,
      graphics::WHITE,
    )?;
    graphics::draw(ctx, &circle, (na::Point2::new(10., 380.0),))?;

    Ok(())
  }

  fn name(&self) -> &str {
    "MenuScene"
  }

  fn input(&mut self, ev: ggez::event::KeyCode, started: bool) {
    if ev == event::KeyCode::Escape && !started {
      self.done = true;
    }
  }
}

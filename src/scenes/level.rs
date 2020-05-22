use ggez::nalgebra as na;
use ggez::{event, graphics, Context, GameResult};

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
      self.done = false;
      scene::SceneSwitch::Pop
    } else {
      scene::SceneSwitch::None
    }
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      na::Point2::new(0.0, 0.0),
      10.0,
      2.0,
      graphics::WHITE,
    )?;
    graphics::draw(ctx, &circle, (na::Point2::new(330., 380.0),))?;
    Ok(())
  }

  fn name(&self) -> &str {
    "LevelScene"
  }

  fn input(&mut self, ev: ggez::event::KeyCode, started: bool) {
    if ev == event::KeyCode::Tab && !started {
      self.done = true;
    }
  }
}

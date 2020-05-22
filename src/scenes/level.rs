use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::{event, graphics, Context, GameResult};

use super::drawable::Drawable;
use super::playable::Playable;
use super::scene;
use crate::scenes;
use crate::scenes::player;

#[allow(dead_code)]
const MARGIN_COLOR_1: (f32, f32, f32, f32) = (0.347, 0.602, 0.186, 1.00);
#[allow(dead_code)]
const RIVER_COLOR: (f32, f32, f32, f32) = (0.169, 0.159, 0.747, 1.00);
#[allow(dead_code)]
const MENU_COLOR_1: (f32, f32, f32, f32) = (0.659, 0.659, 0.659, 1.00);

const GRID_SIZE: (usize, usize) = (10, 10);
const GRID_CELL_SIZE: (usize, usize) = (64, 64);
const SCREEN_SIZE: (f32, f32) = (
  GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
  GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

pub struct LevelScene {
  done: bool,
  drawables: Vec<Box<dyn Drawable>>,
  player: Box<dyn Playable>,
}

impl LevelScene {
  pub fn new(ctx: &mut Context) -> Self {
    let done = false;
    let drawables = vec![];
    let x = (SCREEN_SIZE.0 / 2.) - 20.;
    let y = SCREEN_SIZE.1 - 60.;
    let player = Box::new(player::Player::new(ctx, x, y));

    LevelScene {
      done,
      drawables,
      player,
    }
  }
}

impl scene::Scene<event::KeyCode> for LevelScene {
  fn update(&mut self, ctx: &mut Context) -> scenes::Switch {
    for d in self.drawables.iter_mut() {
      d.update(ctx);
    }

    self.player.update(ctx);

    if self.done {
      event::quit(ctx);
      scene::SceneSwitch::None
    } else {
      scene::SceneSwitch::None
    }
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let bg = graphics::Mesh::new_rectangle(
      ctx,
      graphics::DrawMode::fill(),
      ggez::graphics::Rect::new(0., 0., SCREEN_SIZE.0, SCREEN_SIZE.1),
      graphics::Color::from(RIVER_COLOR),
    )?;

    graphics::draw(ctx, &bg, DrawParam::default())?;

    for d in self.drawables.iter_mut() {
      d.draw(ctx)?;
    }

    self.player.draw(ctx)?;

    Ok(())
  }

  fn name(&self) -> &str {
    "LevelScene"
  }

  fn input(&mut self, ev: ggez::event::KeyCode, started: bool) {
    if ev == event::KeyCode::Escape && !started {
      self.done = true;
    }

    self.player.input(ev, started);
  }
}

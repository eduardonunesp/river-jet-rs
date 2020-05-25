use cgmath::Vector2;
use ggez::graphics::{DrawParam, Rect};
use ggez::{event, graphics, Context, GameResult};
use rand;
use rand::Rng;

use super::mechanics::drawable::Drawable;
use super::mechanics::playable::Playable;
use super::player;
use super::scene;
use super::ship;
use crate::scenes;

#[allow(dead_code)]
const MARGIN_COLOR_1: (f32, f32, f32, f32) = (0.368, 0.605, 0.272, 1.00);
#[allow(dead_code)]
const RIVER_COLOR: (f32, f32, f32, f32) = (0.224, 0.237, 0.717, 1.00);
#[allow(dead_code)]
const MENU_COLOR_1: (f32, f32, f32, f32) = (0.659, 0.659, 0.659, 1.00);

const RIVER_SEGMENT: f32 = 640. * 5.;

const TICK: f32 = 0.1;
const MARGIN_Y_MOVE: f32 = 10.;

const ENEMY_TICK: f32 = 0.010;
const ENEMY_Y_MOVE: f32 = 0.5;

#[allow(dead_code)]
const MARGIN_Y_HEIGHT: f32 = 20.;

const GRID_SIZE: (usize, usize) = (10, 10);
const GRID_CELL_SIZE: (usize, usize) = (64, 64);
const SCREEN_SIZE: (f32, f32) = (
  GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
  GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

struct Margin {
  rect: Rect,
}

pub struct LevelScene {
  done: bool,
  drawables: Vec<Box<dyn Drawable>>,
  player: Box<dyn Playable>,
  left_margins: Vec<Margin>,
  center_margins: Vec<Margin>,
  right_margins: Vec<Margin>,
  segment_completed: f32,
  segment_variations: Vec<f32>,
  tick: f32,
  enemy_tick: f32,
}

impl LevelScene {
  pub fn new(ctx: &mut Context) -> Self {
    let done = false;
    let enemy: Box<dyn Drawable> = Box::new(ship::Ship::new(ctx, 200., 100.));
    let drawables = vec![enemy];
    let mut left_margins = vec![];
    let center_margins = vec![];
    let mut right_margins = vec![];
    let segment_variations = vec![];
    let x = (SCREEN_SIZE.0 / 2.) - 20.;
    let y = SCREEN_SIZE.1 - 70.;
    let player = Box::new(player::Player::new(ctx, x, y));

    // let mut rng = rand::thread_rng();
    // let rand_segments = rng.gen_range::<f32, f32, f32>(0., RIVER_SEGMENT);
    for i in 0..30 {
      left_margins.push(Margin {
        rect: Rect {
          x: 0.,
          y: i as f32 * MARGIN_Y_HEIGHT,
          w: 100.,
          h: MARGIN_Y_HEIGHT,
        },
      });

      right_margins.push(Margin {
        rect: Rect {
          x: 640. - 100.,
          y: i as f32 * MARGIN_Y_HEIGHT,
          w: 100.,
          h: MARGIN_Y_HEIGHT,
        },
      });
    }

    LevelScene {
      done,
      left_margins,
      center_margins,
      right_margins,
      drawables,
      player,
      tick: TICK,
      segment_completed: 0.,
      segment_variations,
      enemy_tick: ENEMY_TICK,
    }
  }

  fn draw_background(&mut self, ctx: &mut Context) -> GameResult<()> {
    let bg = graphics::Mesh::new_rectangle(
      ctx,
      graphics::DrawMode::fill(),
      ggez::graphics::Rect::new(0., 0., SCREEN_SIZE.0, SCREEN_SIZE.1),
      graphics::Color::from(RIVER_COLOR),
    )?;

    graphics::draw(ctx, &bg, DrawParam::default())?;

    Ok(())
  }

  fn draw_margins(&mut self, ctx: &mut Context) -> GameResult<()> {
    for lm in self.left_margins.iter() {
      let margin_line = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        lm.rect,
        graphics::Color::from(MARGIN_COLOR_1),
      )?;

      graphics::draw(ctx, &margin_line, DrawParam::default())?;
    }

    for cm in self.center_margins.iter() {
      let margin_line = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        cm.rect,
        graphics::Color::from(MARGIN_COLOR_1),
      )?;

      graphics::draw(ctx, &margin_line, DrawParam::default())?;
    }

    for rm in self.right_margins.iter() {
      let margin_line = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        rm.rect,
        graphics::Color::from(MARGIN_COLOR_1),
      )?;

      graphics::draw(ctx, &margin_line, DrawParam::default())?;
    }

    Ok(())
  }

  fn draw_stuff(&mut self, ctx: &mut Context) -> GameResult<()> {
    for d in self.drawables.iter_mut() {
      d.draw(ctx)?;
    }

    self.player.draw(ctx)?;
    Ok(())
  }

  fn update_margins(&mut self) {
    self.left_margins.push(Margin {
      rect: Rect {
        x: 0.,
        y: -MARGIN_Y_HEIGHT,
        w: 100.,
        h: MARGIN_Y_HEIGHT,
      },
    });

    self.right_margins.push(Margin {
      rect: Rect {
        x: 640. - 100.,
        y: -MARGIN_Y_HEIGHT,
        w: 100.,
        h: MARGIN_Y_HEIGHT,
      },
    });
    for lm in self.left_margins.iter_mut() {
      lm.rect.y += MARGIN_Y_MOVE;
    }

    for cm in self.center_margins.iter_mut() {
      cm.rect.y += MARGIN_Y_MOVE;
    }

    for rm in self.right_margins.iter_mut() {
      rm.rect.y += MARGIN_Y_MOVE;
    }

    self.left_margins.retain(|v| v.rect.y < 640.);
    self.center_margins.retain(|v| v.rect.y < 640.);
    self.right_margins.retain(|v| v.rect.y < 640.);
  }

  fn update_drawables(&mut self) {
    for d in self.drawables.iter_mut() {
      d.vmove(Vector2::<f32>::new(0., ENEMY_Y_MOVE));
    }
  }
}

impl scene::Scene<event::KeyCode> for LevelScene {
  fn update(&mut self, ctx: &mut Context) -> scenes::Switch {
    const DESIRED_FPS: u32 = 60;
    let frame_secs = 1.0 / (DESIRED_FPS as f32);

    println!("{}", frame_secs);

    self.tick -= frame_secs;
    if self.tick < 0.0 {
      self.tick = TICK;
      self.update_drawables();
      self.update_margins();
    }

    self.enemy_tick -= frame_secs;
    if self.enemy_tick < 0.0 {
      self.enemy_tick = ENEMY_TICK;
      self.update_drawables();
    }

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
    self.draw_background(ctx)?;
    self.draw_margins(ctx)?;
    self.draw_stuff(ctx)?;
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

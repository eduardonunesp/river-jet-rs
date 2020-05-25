use cgmath::Vector2;
use ggez::graphics::Image;
use ggez::{event, event::KeyCode, input, Context};

use super::mechanics::drawable::load_image;
use super::mechanics::drawable::Drawable;
use super::mechanics::playable::Playable;
use super::mechanics::updatable::Updatable;
use super::mechanics::updatable::{self};

const H_MAX_SPEED: f32 = 6.;
const H_ACC: f32 = 4.0;

const V_ACC: f32 = 0.2;
const V_MAX_SPEED: f32 = 2.0;
const V_MIN_SPEED: f32 = 0.2;

pub struct Player {
  image: Image,
  velocity: Vector2<f32>,
  v_acceleration: f32,
  position: Vector2<f32>,
}

impl Player {
  pub fn new(ctx: &mut Context, x: f32, y: f32) -> Self {
    let image = load_image(ctx, "player.png".to_string());
    let position = Vector2::<f32>::new(x, y);
    let velocity = updatable::VECTOR_ZERO;
    let v_acceleration = 0.0;

    Player {
      image,
      position,
      v_acceleration,
      velocity,
    }
  }

  #[allow(dead_code)]
  fn get_acceleration(&self) -> f32 {
    self.v_acceleration
  }
}

impl Updatable for Player {
  fn update(&mut self, ctx: &mut Context) {
    self.velocity = updatable::VECTOR_ZERO;

    if input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
      self.velocity = updatable::VECTOR_LEFT;
    } else if input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
      self.velocity = updatable::VECTOR_RIGHT;
    }

    self.position += self.velocity * H_MAX_SPEED;
  }

  fn vmove(&mut self, move_vec: Vector2<f32>) {}
}

impl Playable for Player {
  fn input(&mut self, ev: event::KeyCode, _started: bool) {
    match ev {
      KeyCode::Up => self.v_acceleration += V_ACC,
      KeyCode::Down => self.v_acceleration -= V_ACC,
      _ => (),
    }

    self.v_acceleration = self.v_acceleration.min(V_MIN_SPEED);
    self.v_acceleration = self.v_acceleration.max(V_MAX_SPEED);
  }
}

impl Drawable for Player {
  fn get_position(&self) -> &Vector2<f32> {
    &self.position
  }

  fn get_image(&self) -> &Image {
    &self.image
  }
}

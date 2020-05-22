use cgmath::Vector2;
use ggez::graphics::Image;
use ggez::{event, event::KeyCode, Context};

use super::drawable::load_image;
use super::drawable::Drawable;
use super::playable::Playable;
use super::updatable::Updatable;
use super::updatable::{self};

const PLAYER_SIDE_SPEED: f32 = 1.;
const ACCELERATION_RATION: f32 = 0.2;
const MAX_ACCELERATION: f32 = 2.0;
const MIN_ACCELERATION: f32 = 0.2;

pub struct Player {
  image: Image,
  velocity: Vector2<f32>,
  acceleration: f32,
  position: Vector2<f32>,
}

impl Player {
  pub fn new(ctx: &mut Context, x: f32, y: f32) -> Self {
    let image = load_image(ctx, "player.png".to_string());
    let position = Vector2::<f32>::new(x, y);
    let velocity = updatable::VECTOR_ZERO;
    let acceleration = 10.0;

    Player {
      image,
      position,
      acceleration,
      velocity,
    }
  }

  fn get_acceleration(&self) -> f32 {
    self.acceleration
  }
}

impl Updatable for Player {
  fn update(&mut self, _ctx: &mut Context) {
    self.position += self.velocity;
  }
}

impl Playable for Player {
  fn input(&mut self, ev: event::KeyCode, started: bool) {
    if !started {
      self.velocity = updatable::VECTOR_ZERO
    } else {
      self.velocity = match ev {
        KeyCode::Right => updatable::VECTOR_RIGHT * PLAYER_SIDE_SPEED,
        KeyCode::Left => updatable::VECTOR_LEFT * PLAYER_SIDE_SPEED,
        _ => updatable::VECTOR_ZERO,
      };
    }

    match ev {
      KeyCode::Up => self.acceleration += ACCELERATION_RATION,
      KeyCode::Down => self.acceleration -= ACCELERATION_RATION,
      _ => (),
    }

    self.acceleration = self.acceleration.min(MIN_ACCELERATION);
    self.acceleration = self.acceleration.max(MAX_ACCELERATION);
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

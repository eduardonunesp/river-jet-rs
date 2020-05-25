use cgmath::Vector2;
use ggez::graphics::Image;
use ggez::Context;

use super::mechanics::drawable::load_image;
use super::mechanics::drawable::Drawable;
use super::mechanics::updatable::Updatable;
use super::mechanics::updatable::{self};

const H_MAX_SPEED: f32 = 6.;

pub struct Ship {
  image: Image,
  velocity: Vector2<f32>,
  position: Vector2<f32>,
}

impl Ship {
  pub fn new(ctx: &mut Context, x: f32, y: f32) -> Self {
    let image = load_image(ctx, "ship.png".to_string());
    let position = Vector2::<f32>::new(x, y);
    let velocity = updatable::VECTOR_ZERO;

    Ship {
      image,
      position,
      velocity,
    }
  }
}

impl Updatable for Ship {
  fn update(&mut self, _ctx: &mut Context) {
    self.velocity = updatable::VECTOR_ZERO;
    self.position += self.velocity * H_MAX_SPEED;
  }

  fn vmove(&mut self, move_vec: Vector2<f32>) {
    self.position += move_vec;
  }
}

impl Drawable for Ship {
  fn get_position(&self) -> &Vector2<f32> {
    &self.position
  }

  fn get_image(&self) -> &Image {
    &self.image
  }
}

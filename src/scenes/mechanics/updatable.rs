use cgmath::Vector2;
use ggez::Context;

pub const VECTOR_RIGHT: Vector2<f32> = Vector2::new(1., 0.);
pub const VECTOR_LEFT: Vector2<f32> = Vector2::new(-1., 0.);
pub const VECTOR_DOWN: Vector2<f32> = Vector2::new(0., 1.);
pub const VECTOR_UP: Vector2<f32> = Vector2::new(0., -1.);
pub const VECTOR_ZERO: Vector2<f32> = Vector2::new(0., 0.);

pub trait Updatable {
  fn update(&mut self, ctx: &mut Context);
}

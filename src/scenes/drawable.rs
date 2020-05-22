use cgmath::Vector2;
use ggez::graphics::Image;
use ggez::{graphics, Context, GameResult};

use super::updatable::Updatable;

pub fn load_image(ctx: &mut Context, image_name: String) -> Image {
  Image::new(ctx, format!("/images/{}", image_name)).unwrap()
}

pub trait Drawable: Updatable {
  fn get_position(&self) -> &Vector2<f32>;
  fn get_image(&self) -> &Image;

  fn draw(&self, ctx: &mut Context) -> GameResult {
    graphics::draw(
      ctx,
      self.get_image(),
      (ggez::mint::Point2 {
        x: (self.get_position().x),
        y: (self.get_position().y),
      },),
    )?;

    Ok(())
  }
}

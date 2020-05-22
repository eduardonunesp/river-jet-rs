use ggez::event;

use super::drawable::Drawable;

pub trait Playable: Drawable {
  fn input(&mut self, ev: event::KeyCode, started: bool);
}

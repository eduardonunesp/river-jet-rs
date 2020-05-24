use super::scene;

pub mod level;
mod mechanics;
pub mod menu;
pub mod player;

// Shortcuts for our scene type.
pub type Switch = scene::SceneSwitch<ggez::event::KeyCode>;
pub type Stack = scene::SceneStack<ggez::event::KeyCode>;

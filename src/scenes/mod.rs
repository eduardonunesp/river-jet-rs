use super::scene;

mod drawable;
pub mod level;
pub mod menu;
mod playable;
pub mod player;
mod updatable;

// Shortcuts for our scene type.
pub type Switch = scene::SceneSwitch<ggez::event::KeyCode>;
pub type Stack = scene::SceneStack<ggez::event::KeyCode>;

use super::scene;

pub mod level;
pub mod menu;

// Shortcuts for our scene type.
pub type Switch = scene::SceneSwitch<ggez::event::KeyCode>;
pub type Stack = scene::SceneStack<ggez::event::KeyCode>;

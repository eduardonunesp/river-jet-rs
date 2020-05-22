use super::scene;

pub mod level;

// Shortcuts for our scene type.
pub type Switch = scene::SceneSwitch<ggez::event::KeyCode>;
pub type Stack = scene::SceneStack<ggez::event::KeyCode>;

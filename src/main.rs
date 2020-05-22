use std::env;
use std::path;

use ggez::event::EventHandler;
use ggez::{event, graphics, timer, Context, GameResult};

mod scene;
mod scenes;

const GRID_SIZE: (usize, usize) = (10, 10);
const GRID_CELL_SIZE: (usize, usize) = (64, 64);
const SCREEN_SIZE: (f32, f32) = (
  GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
  GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

struct GameState {
  scenes: scenes::Stack,
}

impl GameState {
  pub fn new(ctx: &mut Context) -> GameResult<GameState> {
    let mut scenestack = scenes::Stack::new(ctx);
    let menu_scene = Box::new(scenes::level::LevelScene::new(ctx));
    scenestack.push(menu_scene);

    Ok(GameState { scenes: scenestack })
  }
}

impl EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    const DESIRED_FPS: u32 = 60;
    while timer::check_update_time(ctx, DESIRED_FPS) {
      self.scenes.update(ctx);
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, graphics::BLACK);
    self.scenes.draw(ctx);
    graphics::present(ctx)
  }

  fn key_down_event(
    &mut self,
    _ctx: &mut Context,
    keycode: event::KeyCode,
    _keymod: event::KeyMods,
    _repeat: bool,
  ) {
    self.scenes.input(keycode, true);
  }

  fn key_up_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymod: event::KeyMods) {
    self.scenes.input(keycode, false);
  }
}

fn main() -> GameResult {
  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };

  let (ctx, events_loop) =
    &mut ggez::ContextBuilder::new("river-jet-rs", "eduardonunesp@gmail.com")
      .window_setup(ggez::conf::WindowSetup::default().title("river-jet-rs"))
      .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
      .backend(ggez::conf::Backend::OpenGL { major: 3, minor: 2 })
      .add_resource_path(resource_dir)
      .build()?;

  let state = &mut GameState::new(ctx)?;
  event::run(ctx, events_loop, state)
}

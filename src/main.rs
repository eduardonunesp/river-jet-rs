use std::env;
use std::path;

use ggez::event::EventHandler;
use ggez::{event, graphics, Context, GameResult};

const GRID_SIZE: (usize, usize) = (10, 10);
const GRID_CELL_SIZE: (usize, usize) = (64, 64);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

struct GameState {
    // Your state here...
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult<GameState> {
        // Load/create resources such as images here.
        Ok(GameState {
            // ...
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // Draw code here...
        graphics::present(ctx)
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

use ggez;

/// Based on ggez goodies scene manager

/// A command to change to a new scene, either by pushing a new one,
/// popping one or replacing the current scene (pop and then push).
#[allow(dead_code)]
pub enum SceneSwitch<Ev> {
  None,
  Push(Box<dyn Scene<Ev>>),
  Replace(Box<dyn Scene<Ev>>),
  Pop,
}

pub trait Scene<Ev> {
  fn update(&mut self, ctx: &mut ggez::Context) -> SceneSwitch<Ev>;
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()>;
  fn input(&mut self, event: Ev, started: bool);
  fn name(&self) -> &str;
  fn draw_previous(&self) -> bool {
    false
  }
}

#[allow(dead_code)]
impl<Ev> SceneSwitch<Ev> {
  pub fn replace<S>(scene: S) -> Self
  where
    S: Scene<Ev> + 'static,
  {
    SceneSwitch::Replace(Box::new(scene))
  }

  pub fn push<S>(scene: S) -> Self
  where
    S: Scene<Ev> + 'static,
  {
    SceneSwitch::Push(Box::new(scene))
  }

  pub fn pop() -> Self {
    SceneSwitch::Pop
  }
}

pub struct SceneStack<Ev> {
  scenes: Vec<Box<dyn Scene<Ev>>>,
}

#[allow(dead_code)]
impl<Ev> SceneStack<Ev> {
  pub fn new(_ctx: &mut ggez::Context) -> Self {
    Self { scenes: Vec::new() }
  }

  pub fn push(&mut self, scene: Box<dyn Scene<Ev>>) {
    self.scenes.push(scene)
  }

  pub fn pop(&mut self) -> Box<dyn Scene<Ev>> {
    self
      .scenes
      .pop()
      .expect("ERROR: Popped an empty scene stack.")
  }

  pub fn current(&self) -> &dyn Scene<Ev> {
    &**self
      .scenes
      .last()
      .expect("ERROR: Tried to get current scene of an empty scene stack.")
  }

  pub fn switch(&mut self, next_scene: SceneSwitch<Ev>) -> Option<Box<dyn Scene<Ev>>> {
    match next_scene {
      SceneSwitch::None => None,
      SceneSwitch::Pop => {
        let s = self.pop();
        println!("POP scene {}", s.name());
        Some(s)
      }
      SceneSwitch::Push(s) => {
        println!("PUSH scene {}", s.name());
        self.push(s);
        None
      }
      SceneSwitch::Replace(s) => {
        let old_scene = self.pop();
        println!("REPLACE scene {}", s.name());
        self.push(s);
        Some(old_scene)
      }
    }
  }

  pub fn update(&mut self, ctx: &mut ggez::Context) {
    let next_scene = {
      let current_scene = &mut **self
        .scenes
        .last_mut()
        .expect("Tried to update empty scene stack");
      current_scene.update(ctx)
    };
    self.switch(next_scene);
  }

  fn draw_scenes(scenes: &mut [Box<dyn Scene<Ev>>], ctx: &mut ggez::Context) {
    assert!(scenes.len() > 0);
    if let Some((current, rest)) = scenes.split_last_mut() {
      if current.draw_previous() {
        SceneStack::draw_scenes(rest, ctx);
      }
      current
        .draw(ctx)
        .expect("I would hope drawing a scene never fails!");
    }
  }

  pub fn draw(&mut self, ctx: &mut ggez::Context) {
    SceneStack::draw_scenes(&mut self.scenes, ctx)
  }

  pub fn input(&mut self, event: Ev, started: bool) {
    let current_scene = &mut **self
      .scenes
      .last_mut()
      .expect("Tried to do input for empty scene stack");
    current_scene.input(event, started);
  }
}

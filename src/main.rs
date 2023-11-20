extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

mod components;
mod entities;
mod game;
mod libs;

use std::env::current_dir;

use components::*;
use game::*;
use libs::config::{FONT_NAME, WINDOW_HEIGHT, WINDOW_NAME, WINDOW_WIDTH};

use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, ButtonState, EventLoop};

use piston_window::PistonWindow as Window;

// #[derive(Copy, Clone, Hash, PartialEq, Eq)]
// enum Music {
//   MainTheme,
// }

// #[derive(Copy, Clone, Hash, PartialEq, Eq)]
// enum Sound {}

fn main() {
  clearscreen::clear().expect("failed to clear screen");

  let opengl = OpenGL::V3_2;
  let assets = current_dir().unwrap().join("assets");
  let font = assets.join("fonts").join(FONT_NAME);
  let mut window: Window = WindowSettings::new(WINDOW_NAME, [WINDOW_WIDTH, WINDOW_HEIGHT])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .fullscreen(false)
    .resizable(false)
    .vsync(true)
    .automatic_close(true)
    .build()
    .unwrap();

  window.set_lazy(true);

  let mut game_manager = GameManager::start(
    GlGraphics::new(opengl),
    GlyphCache::new(font, (), TextureSettings::new()).unwrap(),
    &assets,
  );
  let mut events = Events::new(EventSettings::new()).ups(60);

  while let Some(e) = events.next(&mut window) {
    e.update(|args| game_manager.update(args));
    e.render(|args| game_manager.render(args));

    if let Some(keys) = e.button_args() {
      if keys.state == ButtonState::Press {
        game_manager.on_press(&keys);
      }
    }
  }
}

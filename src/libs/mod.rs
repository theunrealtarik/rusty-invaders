pub mod assets;
pub mod config;
pub mod intervals;
pub mod sounds;
pub mod textures;

pub mod utils {
  use cgmath::Vector2;
  use graphics::types::{Color, FontSize};
  use graphics::*;
  use opengl_graphics::{GlGraphics, GlyphCache};

  pub fn draw_text(
    ctx: &Context,
    gl: &mut GlGraphics,
    glyphs: &mut GlyphCache,
    color: Color,
    font_size: FontSize,
    pos: Vector2<f64>,
    text: String,
  ) {
    text::Text::new_color(color, font_size)
      .draw(
        &text,
        glyphs,
        &DrawState::default(),
        ctx.transform.trans(pos.x, pos.y),
        gl,
      )
      .unwrap();
  }
}

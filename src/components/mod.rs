use cgmath::Vector2;
use graphics::types::Matrix2d;
use opengl_graphics::{GlGraphics, Texture};
use piston::{ButtonArgs, RenderArgs, UpdateArgs};

#[allow(unused_variables)]
pub trait Entity {
  fn get_position(&mut self) -> Vector2<f64>;
  fn get_scale(&mut self) -> Vector2<f64>;
  fn update(&mut self, update_args: &UpdateArgs) {}
  fn render(
    &mut self,
    renderer: &RenderArgs,
    texture: &Option<Texture>,
    gl: &mut GlGraphics,
    curr_trans: Matrix2d,
  ) {
    use graphics::*;
    let pos = self.get_position();
    let scale = self.get_scale();

    match texture {
      Some(tex) => image(
        tex,
        curr_trans.trans(pos.x, pos.y).scale(scale.x, scale.y),
        gl,
      ),
      None => {}
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  pub direction: Direction,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}

pub trait Rect {
  fn x(&self) -> f64;
  fn y(&self) -> f64;
  fn xw(&self) -> f64;
  fn yh(&self) -> f64;
  fn center_x(&self) -> f64;
  fn center_y(&self) -> f64;
  fn center(&self) -> Vector2<f64>;
}

impl Rect for Transform {
  fn x(&self) -> f64 {
    self.x
  }

  fn y(&self) -> f64 {
    self.y
  }

  fn xw(&self) -> f64 {
    self.x + self.width
  }

  fn yh(&self) -> f64 {
    self.y + self.height
  }

  fn center_x(&self) -> f64 {
    self.x + self.width / 2.0
  }

  fn center_y(&self) -> f64 {
    self.y + self.height / 2.0
  }

  fn center(&self) -> Vector2<f64> {
    Vector2 {
      x: self.center_x(),
      y: self.center_y(),
    }
  }
}

pub trait Collider {
  fn get_transform(&self) -> &Transform;

  fn collides<T>(&self, collider: &T) -> bool
  where
    T: Collider,
  {
    let local_transform = self.get_transform();
    let rect_transform = collider.get_transform();

    if local_transform.x() <= rect_transform.xw()
      && local_transform.xw() >= rect_transform.x()
      && local_transform.y() <= rect_transform.yh()
      && local_transform.yh() >= rect_transform.y()
    {
      return true;
    }

    false
  }

  fn exceeds(&self, region: &Transform) -> (bool, Option<Direction>) {
    let local_transform = self.get_transform();

    if local_transform.y() <= region.y() {
      (true, Some(Direction::Up))
    } else if local_transform.yh() >= region.yh() {
      (true, Some(Direction::Down))
    } else if local_transform.x() <= region.x() {
      (true, Some(Direction::Left))
    } else if local_transform.xw() >= region.xw() {
      (true, Some(Direction::Right))
    } else {
      (false, None)
    }
  }
}

pub trait Controller {
  fn on_press(&mut self, button_args: &ButtonArgs);
}

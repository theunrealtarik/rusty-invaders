use crate::libs::config::*;
use crate::*;

use cgmath::Vector2;
use piston::{RenderArgs, UpdateArgs};

pub struct Player {
  pub transform: Transform,
  pub scale: Vector2<f64>,
  pub velocity: Vector2<f64>,
  pub health: Health,
}

impl Player {
  pub fn new() -> Self {
    Self {
      transform: Transform {
        x: PLAYER_INIT_X,
        y: PLAYER_INIT_Y,
        width: PLAYER_WIDTH,
        height: PLAYER_HEIGHT,
        direction: Direction::Up,
      },
      scale: Vector2 { x: 1.0, y: 1.0 },
      velocity: PLAYER_VELOCITY,
      health: Health {
        transform: Transform {
          x: 20.0,
          y: 20.0,
          width: HEALTH_BAR_WIDTH,
          height: HEALTH_BAR_HEIGHT,
          direction: Direction::Up,
        },
        value: 100.0,
      },
    }
  }

  pub fn is_alive(&self) -> bool {
    self.health.value > 0.0
  }
}

impl Entity for Player {
  fn update(&mut self, update_args: &UpdateArgs) {
    match self.transform.direction {
      Direction::Right => self.transform.x += self.velocity.x * update_args.dt,
      Direction::Left => self.transform.x -= self.velocity.x * update_args.dt,
      _ => {}
    }

    if self.transform.center_x() < 0.0 {
      self.transform.x = WINDOW_WIDTH - self.transform.width / 2.0
    } else if self.transform.x() > WINDOW_WIDTH {
      self.transform.x = self.transform.width / 2.0;
    }

    self.health.transform.width = self.health.value;
  }

  fn get_position(&mut self) -> Vector2<f64> {
    Vector2 {
      x: self.transform.x,
      y: self.transform.y,
    }
  }

  fn get_scale(&mut self) -> Vector2<f64> {
    self.scale
  }
}

impl Collider for Player {
  fn get_transform(&self) -> &Transform {
    &self.transform
  }
}

pub struct Health {
  pub value: f64,
  pub transform: Transform,
}

impl Health {
  pub fn render(
    &mut self,
    _: &RenderArgs,
    gl: &mut GlGraphics,
    curr_trans: graphics::types::Matrix2d,
  ) {
    use graphics::*;

    let bar_background = rectangle::rectangle_by_corners(
      self.transform.x,
      self.transform.y,
      self.transform.x + HEALTH_BAR_WIDTH + HEALTH_BAR_STROKE,
      self.transform.y + HEALTH_BAR_HEIGHT + HEALTH_BAR_STROKE,
    );

    let bar = rectangle::rectangle_by_corners(
      self.transform.x + HEALTH_BAR_STROKE,
      self.transform.y + HEALTH_BAR_STROKE,
      self.transform.xw(),
      self.transform.yh(),
    );

    rectangle(color::BLACK, bar_background, curr_trans, gl);
    rectangle(color::GREEN, bar, curr_trans, gl);
  }
}

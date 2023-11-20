use std::time::Instant;

use cgmath::Vector2;
use piston::UpdateArgs;

use crate::{
  components::*,
  libs::config::{BULLET_HEIGHT, BULLET_WIDTH},
};

#[derive(Debug, Clone, Copy)]
pub enum Shooter {
  Player,
  Enemy,
}

#[derive(Debug, Clone)]
pub struct Bullet {
  pub transform: Transform,
  pub scale: Vector2<f64>,
  pub acceleration: Vector2<f64>,
  pub velocity: Vector2<f64>,
  pub damage: f64,
  pub owner: Shooter,
  pub time: Instant,
}

impl Bullet {
  pub fn spawn(
    x: &f64,
    y: &f64,
    scale: Vector2<f64>,
    acceleration: Vector2<f64>,
    velocity: Vector2<f64>,
    damage: f64,
    owner: Shooter,
  ) -> Self {
    Self {
      transform: Transform {
        x: *x - BULLET_WIDTH / 2.0,
        y: *y,
        width: BULLET_WIDTH,
        height: BULLET_HEIGHT,
        direction: Direction::Up,
      },
      scale,
      acceleration,
      velocity,
      damage,
      owner,
      time: Instant::now(),
    }
  }
}

impl Entity for Bullet {
  fn update(&mut self, update_args: &UpdateArgs) {
    let t = self.time.elapsed().as_secs_f64();
    self.transform.y += (0.5) * (self.acceleration.y) * t.powi(2) + self.velocity.y * t;
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

impl Collider for Bullet {
  fn get_transform(&self) -> &Transform {
    &self.transform
  }
}

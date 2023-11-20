use std::rc::Rc;

use cgmath::Vector2;
use graphics::types::Matrix2d;
use opengl_graphics::{GlGraphics, Texture};
use piston::{RenderArgs, UpdateArgs};

use crate::components::*;
use crate::libs::{config::*, textures::EnemiesTextures};

#[derive(Debug, Clone)]
struct EGS {
  velocity: Vector2<f64>,
  zone: Transform,
}

#[derive(Debug, Clone)]
pub struct EnemyManager {
  pub cols: u8,
  pub rows: u8,
  pub entities: Vec<Enemy>,
  pub reached_player: bool,
  state: EGS,
}

impl EnemyManager {
  pub fn new(cols: u8, rows: u8) -> Self {
    Self {
      cols,
      rows,
      entities: EnemyManager::get_entities(cols, rows),
      state: EGS {
        zone: Transform {
          x: 50.0,
          y: 0.0,
          width: WINDOW_WIDTH - 100.0,
          height: WINDOW_HEIGHT - 50.0,
          direction: Direction::Up,
        },
        velocity: Vector2 {
          x: ENEMY_SPEED,
          y: 0.0,
        },
      },
      reached_player: false,
    }
  }

  fn get_entities(c: u8, r: u8) -> Vec<Enemy> {
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut curr_row: f64 = 1.0;
    let mut curr_col: f64 = 1.0;

    let mut kind = EnemyVariation::Squid;

    for _ in 0..(c * r) as usize {
      if curr_row > 1.0 {
        kind = EnemyVariation::Crab
      }

      if curr_row > 3.0 {
        kind = EnemyVariation::Octopus
      }

      if enemies.len() < (c * r) as usize {
        enemies.push(Enemy::new(
          curr_col * (ENEMY_WIDTH + ENEMY_SPACING) - ENEMY_SPACING + ENEMY_GRID_X,
          curr_row * (ENEMY_HEIGHT + ENEMY_SPACING) - ENEMY_SPACING + ENEMY_GRID_Y,
          Vector2 { x: 1.0, y: 1.0 },
          kind,
        ));
      }

      if curr_col == c as f64 {
        curr_row += 1.0;
        curr_col = 0.0;
      }

      curr_col += 1.0;
    }

    enemies
  }

  pub fn render(
    &mut self,
    _renderer: &RenderArgs,
    texture: &EnemiesTextures,
    gl: &mut GlGraphics,
    curr_trans: Matrix2d,
  ) {
    let shared_textures = Rc::new(texture);

    for entity in &mut self.entities {
      match entity.variation {
        EnemyVariation::Crab => {
          entity.render(gl, shared_textures.crab.as_ref().unwrap(), curr_trans)
        }
        EnemyVariation::Squid => {
          entity.render(gl, shared_textures.squid.as_ref().unwrap(), curr_trans)
        }
        EnemyVariation::Octopus => {
          entity.render(gl, shared_textures.octopus.as_ref().unwrap(), curr_trans)
        }
      }
    }
  }

  pub fn update(&mut self, update_args: &UpdateArgs) {
    let mut bounced = false;

    for entity in &mut self.entities {
      let (exceeds, side) = entity.exceeds(&self.state.zone);

      if exceeds && side.is_some() {
        match side.unwrap() {
          Direction::Up => panic!("enemies exceeds their region on the wrong direction"),
          Direction::Down => {
            self.reached_player = true;
            self.state.velocity = Vector2 { x: 0.0, y: 0.0 };
          }
          Direction::Right => {
            self.state.velocity.x = ENEMY_SPEED * -1.0;
            bounced = true;
          }
          Direction::Left => {
            self.state.velocity.x = ENEMY_SPEED * 1.0;
            bounced = true;
          }
        }
      }
    }

    for entity in &mut self.entities {
      entity.transform.x += self.state.velocity.x * update_args.dt;

      if bounced {
        entity.transform.y += ENEMY_HEIGHT;
      }
    }
  }

  pub fn remove(&mut self, index: usize) -> Enemy {
    self.entities.remove(index)
  }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum EnemyVariation {
  Crab,
  Squid,
  Octopus,
}

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
  pub transform: Transform,
  pub scale: Vector2<f64>,
  pub variation: EnemyVariation,
}

impl Enemy {
  pub fn new(x: f64, y: f64, scale: Vector2<f64>, variation: EnemyVariation) -> Self {
    Self {
      transform: Transform {
        x,
        y,
        width: ENEMY_WIDTH,
        height: ENEMY_HEIGHT,
        direction: Direction::Up,
      },
      scale,
      variation,
    }
  }

  fn render(&mut self, gl: &mut GlGraphics, texture: &Texture, curr_trans: Matrix2d) {
    use graphics::*;
    image(
      texture,
      curr_trans.trans(self.transform.x, self.transform.y),
      gl,
    );
  }
}

impl Entity for Enemy {
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

impl Collider for Enemy {
  fn get_transform(&self) -> &Transform {
    &self.transform
  }
}

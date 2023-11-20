use std::path::PathBuf;

use cgmath::Vector2;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{Button, Key, RenderArgs, UpdateArgs};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::components::*;
use crate::entities::*;
use crate::libs::sounds::SoundEffect;
use crate::libs::sounds::SoundsManager;
use crate::libs::utils;
use crate::libs::{config::*, textures::TextureLoader};

pub struct GameManager<'a> {
  gl: GlGraphics,
  glyphs: GlyphCache<'a>,
  textures: TextureLoader,
  sounds: SoundsManager,
  rng: ThreadRng,

  pub player: Player,
  pub bullets: Vec<Bullet>,
  pub enemies: EnemyManager,

  pub score: u64,
  pub rounds: u32,
  pub player_lost: bool,
}

impl<'a> GameManager<'a> {
  pub fn start(gl: GlGraphics, glyphs: GlyphCache<'a>, assets: &PathBuf) -> Self {
    Self {
      gl,
      glyphs,
      textures: TextureLoader::new(assets),
      sounds: SoundsManager::new(assets),
      rng: rand::thread_rng(),

      player: Player::new(),
      bullets: Vec::new(),
      enemies: EnemyManager::new(ENEMY_GRID_COLS, ENEMY_GRID_ROWS),

      score: 0,
      rounds: 0,
      player_lost: false,
    }
  }

  pub fn restart(&mut self) {
    self.player = Player::new();
    self.bullets = Vec::new();
    self.enemies = EnemyManager::new(ENEMY_GRID_COLS, ENEMY_GRID_ROWS);
    self.score = 0;
    self.rounds = 0;
    self.player_lost = false;
  }

  pub fn render(&mut self, renderer: &RenderArgs) {
    use graphics::*;

    self.gl.draw(renderer.viewport(), |c, gl| {
      clear(color::BLACK, gl);

      match &self.textures.background {
        Some(tex) => image(tex, c.transform, gl),
        None => {}
      }

      if !self.player_lost {
        self.player.health.render(renderer, gl, c.transform);
        self
          .player
          .render(renderer, &self.textures.player, gl, c.transform);

        self
          .enemies
          .render(renderer, &self.textures.enemies, gl, c.transform);

        for bullet in &mut self.bullets {
          bullet.render(renderer, &self.textures.bullet, gl, c.transform);
        }
      }
    });

    self.render_text(renderer);
  }

  fn render_text(&mut self, renderer: &RenderArgs) {
    use graphics::*;

    self.gl.draw(renderer.viewport(), |c, gl| {
      let mut draw_text = |color, font_size, position, text| {
        utils::draw_text(&c, gl, &mut self.glyphs, color, font_size, position, text)
      };

      if self.player_lost {
        draw_text(
          color::WHITE,
          26,
          Vector2 { x: 20.0, y: 60.0 },
          format!("ROUNDS: {}", self.rounds),
        );
        draw_text(
          color::WHITE,
          26,
          Vector2 { x: 20.0, y: 100.0 },
          format!("SCORE: {}", self.score),
        );
        draw_text(
          color::WHITE,
          26,
          Vector2 { x: 20.0, y: 140.0 },
          String::from("SKILL ISSUE"),
        );
      } else {
        draw_text(
          color::WHITE,
          12,
          Vector2 { x: 20.0, y: 50.0 },
          format!("{}", self.score),
        );
      }
    })
  }

  // game update
  pub fn update(&mut self, update: &UpdateArgs) {
    self.player_lost = !self.player.is_alive() || self.enemies.reached_player;

    // keeping both enemies and play fresh
    self.player.update(update);
    self.enemies.update(update);

    if self.player_lost {
      return;
    }

    // rounds
    if self.enemies.entities.is_empty() {
      self.rounds += 1;
      self.player.health.value += self
        .rng
        .gen_range((HEALTH_REGEN_MIN_VALUE + self.rounds as f64)..HEALTH_REGEN_MAX_VALUE);
      self.enemies = EnemyManager::new(ENEMY_GRID_COLS, ENEMY_GRID_ROWS);
    }

    // difficulty i guess
    if self.rng.gen_ratio(self.rounds + 1, 100) {
      let random_index: usize = self.rng.gen_range(0..self.enemies.entities.len());
      let random_enemy = self.enemies.entities[random_index];

      self.bullets.push(Bullet::spawn(
        &random_enemy.transform.x,
        &random_enemy.transform.y,
        Vector2 { x: 1.0, y: -1.0 },
        ENEMY_BULLET_ACCELERATION,
        ENEMY_BULLET_VELOCITY,
        self.rng.gen_range(ENEMY_MIN_DAMAGE..ENEMY_MAX_DAMAGE),
        Shooter::Enemy,
      ))
    }

    // collision stuff
    let mut bullets_temp: Vec<usize> = Vec::new();
    let mut enemies_temp: Vec<usize> = Vec::new();

    for (i, bullet) in self.bullets.iter_mut().enumerate() {
      bullet.update(update);

      match bullet.owner {
        Shooter::Player => self
          .enemies
          .entities
          .iter_mut()
          .enumerate()
          .for_each(|(j, enemy)| {
            if bullet.collides(enemy) {
              self.score += match enemy.variation {
                EnemyVariation::Crab => ENEMY_CRAB_POINTS,
                EnemyVariation::Squid => ENEMY_SQUID_POINTS,
                EnemyVariation::Octopus => ENEMY_OCTOPUS_POINTS,
              };
              self.sounds.play(SoundEffect::EnemyExplosion, 0.2);
              bullets_temp.push(i);
              enemies_temp.push(j);
            }
          }),
        Shooter::Enemy => {
          if bullet.collides(&self.player) {
            self.player.health.value -= bullet.damage;
            self.sounds.play(SoundEffect::PlayerHit, 1.0);
            if !self.player.is_alive() {
              self.sounds.play(SoundEffect::PlayerExplosion, 1.0);
            }

            bullets_temp.push(i);
            break;
          }
        }
      }
    }

    for i in bullets_temp {
      self.bullets.remove(i);
    }

    for i in enemies_temp {
      self.enemies.remove(i);
    }

    // out of bounds bullets
    self
      .bullets
      .retain(|bullet| bullet.transform.y < WINDOW_HEIGHT && bullet.transform.y >= 0.0);
  }
}

impl Controller for GameManager<'_> {
  fn on_press(&mut self, button_args: &piston::ButtonArgs) {
    match button_args.button {
      Button::Keyboard(Key::Right) => self.player.transform.direction = Direction::Right,
      Button::Keyboard(Key::Left) => self.player.transform.direction = Direction::Left,
      Button::Keyboard(Key::R) => {
        if self.player_lost {
          self.restart()
        }
      }
      Button::Keyboard(Key::Space) => {
        let player_bullets = self.bullets.iter().filter(|bullet| match bullet.owner {
          Shooter::Player => true,
          Shooter::Enemy => false,
        });

        let max_bullets = ((self.rounds / PLAYER_BULLETS_FREQUENCY) + 1) as usize;
        if player_bullets.count() < max_bullets && self.player.is_alive() {
          self.bullets.push(Bullet::spawn(
            &(self.player.transform.x + self.player.transform.width / 2.0),
            &self.player.transform.y,
            Vector2 { x: 1.0, y: 1.0 },
            PLAYER_BULLET_ACCELERATION,
            PLAYER_BULLET_VELOCITY,
            100.0,
            Shooter::Player,
          ));
          self.sounds.play(SoundEffect::PlayerLaserShoot, 1.0)
        }
      }
      _ => {}
    }
  }
}

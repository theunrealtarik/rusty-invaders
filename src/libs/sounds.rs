use super::assets::*;
use std::path::PathBuf;

use kira::{
  manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
  sound::static_sound::{StaticSoundData, StaticSoundSettings},
  tween::{Tween, Value},
  Volume,
};

pub enum SoundEffect {
  PlayerExplosion,
  PlayerHit,
  PlayerLaserShoot,
  EnemyExplosion,
}

impl SoundEffect {
  pub fn as_filename(&self) -> &'static str {
    match self {
      SoundEffect::PlayerExplosion => PLAYER_EXPLOSION_SOUND_NAME,
      SoundEffect::PlayerHit => PLAYER_HIT_SOUND_NAME,
      SoundEffect::PlayerLaserShoot => PLAYER_LASER_SHOOT_SOUND_NAME,
      SoundEffect::EnemyExplosion => ENEMY_EXPLOSION_SOUND_NAME,
    }
  }
}

pub struct SoundsManager {
  sounds_path: PathBuf,
  manager: AudioManager,
}

impl SoundsManager {
  pub fn new(assets: &PathBuf) -> Self {
    let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

    Self {
      sounds_path: assets.join("sounds"),
      manager,
    }
  }

  pub fn play(&mut self, sound_effect: SoundEffect, volume: impl Into<Value<Volume>>) {
    let path = self.sounds_path.clone();
    let sound_data = StaticSoundData::from_file(
      path.join(sound_effect.as_filename()),
      StaticSoundSettings::default(),
    )
    .unwrap();

    self
      .manager
      .play(sound_data)
      .unwrap()
      .set_volume(volume, Tween::default())
      .unwrap();
  }
}

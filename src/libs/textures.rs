use std::path::PathBuf;

use opengl_graphics::{Texture, TextureSettings};

use super::assets::*;

pub struct TextureLoader {
  pub player: Option<Texture>,
  pub bullet: Option<Texture>,
  pub background: Option<Texture>,
  pub enemies: EnemiesTextures,
}

pub struct EnemiesTextures {
  pub crab: Option<Texture>,
  pub squid: Option<Texture>,
  pub octopus: Option<Texture>,
}

impl TextureLoader {
  pub fn new(assets_folder: &PathBuf) -> Self {
    let textures = assets_folder.join("images");
    let textures_settings = TextureSettings::new();

    Self {
      player: TextureLoader::handle(Texture::from_path(
        textures.join(PLAYER_TEXTURE_NAME),
        &textures_settings,
      )),
      bullet: TextureLoader::handle(Texture::from_path(
        textures.join(BULLET_TEXTURE_NAME),
        &textures_settings,
      )),
      background: TextureLoader::handle(Texture::from_path(
        textures.join(BACKGROUND_TEXTURE_NAME),
        &textures_settings,
      )),

      enemies: EnemiesTextures {
        crab: TextureLoader::handle(Texture::from_path(
          textures.join(CRAB_TEXTURE_NAME),
          &textures_settings,
        )),
        squid: TextureLoader::handle(Texture::from_path(
          textures.join(SQUID_TEXTURE_NAME),
          &textures_settings,
        )),
        octopus: TextureLoader::handle(Texture::from_path(
          textures.join(OCTOPUS_TEXTURE_NAME),
          &textures_settings,
        )),
      },
    }
  }

  fn handle(tex: Result<Texture, String>) -> Option<Texture> {
    match tex {
      Ok(texture) => Some(texture),
      Err(_) => None,
    }
  }
}

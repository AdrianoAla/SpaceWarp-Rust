use macroquad::audio::*;
use crate::{player::Player};
use macroquad::{prelude::*};
use macroquad_text::Fonts;
use crate::level::get_level;
use lazy_static::lazy_static;

const FONT_SIZE: u16 = 8;

pub fn screen_size() -> i32 {
  return 128;
}  

pub fn _colliding(x1:i32, y1:i32, w1:i32, h1:i32, x2:i32, y2:i32, w2:i32, h2:i32) -> bool {
    if x1 < x2 + w2 &&
       x1 + w1 > x2 &&
       y1 < y2 + h2 &&
       y1 + h1 > y2 {
        return true;
    }
    return false;
}

pub fn get_collision(x:i32, y:i32) -> i32 {
  let mut level = get_level().lock().unwrap();
  for (index, tile) in level.tiles.iter().enumerate() {
    if x >= tile.x && x <= tile.x + tile.width && y >= tile.y && y <= tile.y + tile.height {
      if tile.is_fire() {
        return 2;
      } else {
        if tile.tile_type == '❤' {
          let mut to_remove: Vec<usize> = Vec::new();
          to_remove.push(index);
          for (i, t) in level.tiles.iter().enumerate() {
            if t.is_door() {
              to_remove.push(i-to_remove.len());
            }
          }

          for i in to_remove {
            level.tiles.remove(i);
          }
          play_sound(PICKUP_SOUND.get_sound(), PlaySoundParams {looped:false, volume:0.5});
          return 3;
        }
        return 1;
      }
    }
    if tile.is_door() && x >= tile.x && x <= tile.x + tile.width && y >= tile.y - 8 && y <= tile.y - 8 + tile.height {
      println!("{}, {}, {}, {}", tile.x, tile.y, tile.tile_type, index);
      return 1;
    }
  }
  return -1;
}

pub fn draw_debug_text(fonts:&Fonts, target_fps:&u32, player:&Player) {
  fonts.draw_text(&format!("{} {}", player.get_state().0, player.get_state().1),       1.0, 3.0, FONT_SIZE, WHITE);
  fonts.draw_text(&format!("{:.2}%", (target_fps*100) as f32 / 30.0),                                          1.0, 13.0, FONT_SIZE, WHITE);
}


pub struct SoundLoader {
  pub sound: Sound,
}
impl SoundLoader {
  #[tokio::main]
  pub async fn new(path: &str) -> SoundLoader {
    let sound = load_sound(path).await.unwrap();
    SoundLoader {
      sound,
    }
  }

  pub fn get_sound(&self) -> Sound {
    self.sound
  }
}

pub struct ImageLoader {
  pub texture: Texture2D,
}
impl ImageLoader {
  #[tokio::main]
  pub async fn new(path: &str) -> ImageLoader {
    let texture = load_texture(path).await.unwrap();
    ImageLoader {
      texture,
    }
  }

  pub fn get_texture(&self) -> Texture2D {
    self.texture
  }
}

lazy_static! {
  static ref PICKUP_SOUND:SoundLoader = SoundLoader::new("assets/sounds/item.wav");
}
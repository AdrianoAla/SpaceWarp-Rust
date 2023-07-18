use macroquad::{audio::*};

use crate::{player::Player, tile::*};

use macroquad::prelude::*;
use macroquad_text::Fonts;

use crate::level::get_level;
use lazy_static::lazy_static;

use ini::Ini;

pub enum CollisionState {
  Solid,
  Kill,
  Interactable,
  None
}

const FONT_SIZE: u16 = 8;

pub fn screen_size() -> i32 {
  return 128;
}  

pub fn get_collision(x:i32, y:i32) -> CollisionState {
  
  let mut level = get_level().lock().unwrap();

  let tile_x = (x / 8) as usize;
  let tile_y = (y / 8) as usize;

  if tile_x == 16 || tile_y == 16 {return CollisionState::None};

  if !level.tiles[tile_y][tile_x].collidable { return CollisionState::None };

  let safe_door_y = tile_y + 1;

  if level.tiles[tile_y][tile_x].is_object() {
    if level.tiles[tile_y][tile_x].is_fire() {
      return CollisionState::Kill;
    }

    match level.tiles[tile_y][tile_x].is_key() {
      ObjectColor::None => {},
      key_color => {

        level.tiles[tile_y][tile_x] = Tile::new('⬜');

        play_sound(KEY_SOUND.sound, PlaySoundParams {looped:false, volume:0.5});

        for row in level.tiles.iter_mut() {
          for tile in row.iter_mut() {
            
            match tile.is_door() {
              ObjectColor::None => {},
              door_color => {
                if door_color == key_color {
                  tile.collidable = false;
                  tile.timer = -1;
                  tile.anim_timer = 8;
                  tile.oy = 0;
                  tile.locked = true;
                }
              },
            }
            
          }
        }

        return CollisionState::Interactable;
      }
    }

    match level.tiles[tile_y][tile_x].is_button() {
      ObjectColor::None => {},
      button_color => {

        level.tiles[tile_y][tile_x].collidable = false;
        level.tiles[tile_y][tile_x].anim_timer = 8;
        level.tiles[tile_y][tile_x].oy = 0;
        level.tiles[tile_y][tile_x].timer = 150;

        play_sound(BUTTON_SOUND.sound, PlaySoundParams {looped:false, volume:0.5});

        for row in level.tiles.iter_mut() {
          for tile in row.iter_mut() {
            
            match tile.is_door() {
              ObjectColor::None => {},
              door_color => {
                if door_color == button_color && !tile.locked {
                  tile.collidable = false;
                  tile.timer = 150;
                  tile.anim_timer = 8;
                  tile.oy = 0;
                }
              },
            }
            
          }
        }

        return CollisionState::Interactable;
      }
    }

    return CollisionState::Solid;
  }

  if safe_door_y < 16 {
    match level.tiles[safe_door_y][tile_x].is_door() {
      ObjectColor::None => {},
      _ => { 
        if level.tiles[safe_door_y][tile_x].collidable {
          return CollisionState::Solid; 
        }
      }
    }
  }

  return CollisionState::None;
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
}

lazy_static! {
  static ref KEY_SOUND:SoundLoader = SoundLoader::new("assets/sounds/item.wav");
  static ref BUTTON_SOUND:SoundLoader = SoundLoader::new("assets/sounds/button.wav");

  pub static ref PACK: String = String::from(Ini::load_from_file("config.ini").unwrap().section(Some("Pack")).unwrap().get("name").unwrap());
}

pub fn is_kill(col: CollisionState) -> bool {
  match col {
    CollisionState::Kill => true,
    _ => false
  }
}

pub fn is_solid(col: CollisionState) -> bool {
  match col {
    CollisionState::Solid => true,
    _ => false
  }
}
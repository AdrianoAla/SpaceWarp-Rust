
use crate::{player::Player};
use macroquad::{prelude::*};
use macroquad_text::Fonts;
use crate::level::get_level;

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
  for tile in get_level().get_tiles() {
    if x >= tile.x && x <= tile.x + tile.width && y >= tile.y && y <= tile.y + tile.height {
      return tile.get_state().4;
    }
  }
  return -1;
}

pub fn draw_debug_text(player:&Player, fonts:&Fonts) {

  // calculate which screen the player is on based on location knowing that one screen is 128x128
  let screen_x = (player.x + (player.x%screen_size())) / screen_size();
  let screen_y = (player.y + (player.y%screen_size())) / screen_size();

  fonts.draw_text(&format!("{} {}", player.get_state().0, player.get_state().1),       1.0, 3.0, FONT_SIZE, WHITE);
  fonts.draw_text(&format!("{} {}", screen_x, screen_y),                               1.0, 13.0, FONT_SIZE, WHITE)
}
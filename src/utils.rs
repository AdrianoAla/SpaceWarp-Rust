
use std::fs::File;
use std::io::prelude::*;
use crate::{tile::Tile, player::Player};
use macroquad::{prelude::*};

pub fn load_tiles_from_file() -> Vec<Tile> {

  let file_name = "level.sw";
  let mut file = File::open(file_name).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  // separate into vec by newline
  let lines: Vec<&str> = contents.split("\n").collect();
  // create new tile for each character in each line

  let mut tiles:Vec<Tile> = Vec::new();

  for (y, line) in lines.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      if c == '#' {
        tiles.push(Tile::new(x as f32 * 50.0, y as f32 * 50.0, BLUE));
      }
    }
  }

  return tiles;

}

pub fn screen_size() -> f32 {
  return 800.0;
}  

pub fn colliding(x1:f32, y1:f32, w1:f32, h1:f32, x2:f32, y2:f32, w2:f32, h2:f32) -> bool {
    if x1 < x2 + w2 &&
       x1 + w1 > x2 &&
       y1 < y2 + h2 &&
       y1 + h1 > y2 {
        return true;
    }
    return false;
}

pub fn get_tiles() -> Vec<Tile> {

  let tiles:Vec<Tile> = load_tiles_from_file();


  return tiles;
}

pub fn draw_debug_text(player:&Player, tiles:&Vec<Tile>, frames:u32) {
  draw_text(&format!("X: {} Y: {}", player.get_state().0, player.get_state().1),       0.0, 20.0, 30.0, BLACK);
  draw_text(&format!("Can Jump: {}", player.get_state().4),                            0.0, 45.0, 30.0, BLACK);
  draw_text(&format!("Tiles Loaded: {}", tiles.len()),                                 0.0, 70.0, 30.0, BLACK);
  draw_text(&format!("FPS: {}", get_fps()),                                            0.0, 95.0, 30.0, BLACK);
  draw_text(&format!("Frametime: {}", get_frame_time()),                               0.0, 120.0, 30.0, BLACK);
  draw_text(&format!("Time: {}", get_time()),                                          0.0, 145.0, 30.0, BLACK);
  draw_text(&format!("Time (FPS): {}", frames/60),                                     0.0, 170.0, 30.0, BLACK);
}
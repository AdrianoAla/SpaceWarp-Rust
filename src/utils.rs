
use crate::tile::Tile;
use macroquad::{prelude::*};

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

  // Return the current tiles

  let mut tiles: Vec<Tile> = Vec::new();

  for i in 0..16 {
      tiles.push(Tile::new(50.0 * i as f32, 750.0, BLUE));
  }

  tiles.push(Tile::new(200.0, 700.0, BLUE));
  tiles.push(Tile::new(400.0, 650.0, BLUE));
  tiles.push(Tile::new(600.0, 600.0, BLUE));

  tiles.push(Tile::new(750.0, 600.0, BLUE));
  tiles.push(Tile::new(750.0, 650.0, BLUE));
  tiles.push(Tile::new(750.0, 700.0, BLUE));

  return tiles;
}
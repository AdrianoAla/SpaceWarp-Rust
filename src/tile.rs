use macroquad::{prelude::*};

pub struct Tile {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub color: Color,
  pub tile_type: char
} impl Tile {
  pub fn new(x: i32, y: i32, tile_type:char) -> Tile {

    

    Tile {
      x,
      y,
      width: 8,
      height: 8,
      color: DARKGRAY,
      tile_type,
    }
  }

  pub fn draw(&self) {
    draw_rectangle(self.x as f32, self.y as f32, self.width as f32, self.height as f32, self.color);
  }

  pub fn get_state(&self) -> (i32, i32, i32, i32, char) {
    (self.x, self.y, self.width, self.height, self.tile_type)
  }
}
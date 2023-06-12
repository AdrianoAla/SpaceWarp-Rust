use macroquad::{prelude::*};

pub struct Tile {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
  color: Color,
  tile_type: i32
} impl Tile {
  pub fn new(x: f32, y: f32, color: Color) -> Tile {
    Tile {
      x,
      y,
      width: 50.0,
      height: 50.0,
      color,
      tile_type: 0
    }
  }

  pub fn draw(&self) {
    draw_rectangle(self.x, self.y, self.width, self.height, self.color);
  }

  pub fn get_state(&self) -> (f32, f32, f32, f32, i32) {
    (self.x, self.y, self.width, self.height, self.tile_type)
  }
}
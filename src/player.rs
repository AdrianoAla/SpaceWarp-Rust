use macroquad::{prelude::*};

use crate::utils::*;
use crate::tile::Tile;

pub struct Player {
  pub x: f32,
  pub y: f32,
  width: f32,
  height: f32,
  
  speed: f32,
  
  gravity: f32,
  can_jump: bool,
  jump: f32,
  jump_speed: f32,
  jump_height: f32,
  
  tiles: Vec<Tile>,
}
impl Player {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Player {
    Player {
      x,
      y,
      width,
      height,
      
      speed: 3.4,
      
      gravity: 6.0,
      can_jump: false,
      jump: 0.0,
      jump_height: 25.0,
      jump_speed: 6.0,
      
      tiles: get_tiles(),
    }
  }

  fn colliding_with_tiles(&self) -> bool {
    for tile in &self.tiles {
      if colliding(self.x, self.y, self.width, self.height, tile.x, tile.y, tile.width, tile.height) && tile.get_state().4 == 0 {
        return true;
      }
    }
    return false;
  }

  fn colliding_with_tile(&self, tile: &Tile) -> bool {
    colliding(self.x, self.y, self.width, self.height, tile.x, tile.y, tile.width, tile.height) && tile.get_state().4 == 0
  }

  pub fn draw(&self) {

    draw_rectangle(self.x, self.y, self.width, self.height, RED);
  }

  pub fn update(&mut self) {

    // Gravity

    if self.y + self.height < screen_size() && self.jump == 0.0 {
      self.y += self.gravity;
      for tile in &self.tiles {
        if self.colliding_with_tile(tile) {
          self.y = tile.y - self.height;
          self.jump = 0.0;
          self.can_jump = true;
        }
        
      }
    }

    // Jumping

    if is_key_down(KeyCode::Space) && self.jump == 0.0 && self.can_jump {
      let mut jump = true;
      for tile in &self.tiles {
        if self.colliding_with_tile(tile) {
          if tile.get_state().1 + tile.get_state().3 > self.y {
            jump = false;
          }
        }
      }
      if jump {
        self.jump = self.jump_height;
        self.can_jump = false;
      }
    }

    if self.jump > 0.0 {
      self.y -= self.jump_speed;
      self.jump -= 1.0;
      
      // see if block right above us

      if self.colliding_with_tiles() {
        self.jump = 0.0;
      }
      
    } else if self.jump < 0.0 {
      self.jump = 0.0;
    }

      // Left and right movement

      if is_key_down(KeyCode::Right) {
        self.x += self.speed;
        if self.colliding_with_tiles() {
          self.x -= self.speed;
        }
      }
      if is_key_down(KeyCode::Left) {
        self.x -= self.speed;
        if self.colliding_with_tiles() {
          self.x += self.speed;
        }
      }

      self.x = self.x.round();

  }

  pub fn get_state(&self) -> (f32, f32, f32, f32, bool) {
    (self.x, self.y, self.width, self.height, self.can_jump)
  }

}
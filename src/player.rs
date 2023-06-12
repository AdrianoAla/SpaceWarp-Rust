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
      
      speed: 1.7,
      
      gravity: 3.0,
      can_jump: false,
      jump: 0.0,
      jump_height: 50.0,
      jump_speed: 3.0,
      
      tiles: get_tiles(),
    }
  }

  pub fn draw(&self) {
    draw_rectangle(self.x, self.y, self.width, self.height, RED);
  }

  pub fn update(&mut self) {

    // Gravity

    if self.y + self.height < screen_size() && self.jump == 0.0 {
      self.y += self.gravity;
      for tile in &self.tiles {
        if colliding(self.x, self.y, self.width, self.height, tile.x, tile.y, tile.width, tile.height) {
          self.y = tile.y - self.height;
          self.jump = 0.0;
          self.can_jump = true;
        }
      }
    }

    // Jumping

    if is_key_pressed(KeyCode::Space) && self.jump == 0.0 && self.can_jump {
      self.jump = self.jump_height;
      self.can_jump = false;
    }

    if self.jump > 0.0 {
      self.y -= self.jump_speed;
      self.jump -= 1.0;
      
      // see if block right above us

      for tile in &self.tiles {
        if colliding(self.x, self.y-1.0, self.width, self.height, tile.x, tile.y, tile.width, tile.height) {
          self.jump = 0.0;
        }
      }
      
    } else if self.jump < 0.0 {
      self.jump = 0.0;
    }

      // Left and right movement

      if is_key_down(KeyCode::Right) {
        self.x += self.speed;
        for tile in &self.tiles {
            if colliding(self.x, self.y, self.width, self.height, tile.x, tile.y, tile.width, tile.height) {
              //self.x = tile.x - self.width;
              self.x -= self.speed;
            }
        }
      }
      if is_key_down(KeyCode::Left) {
        self.x -= self.speed;
        for tile in &self.tiles {
          if colliding(self.x, self.y, self.width, self.height, tile.x, tile.y, tile.width, tile.height) {
            //self.x = tile.x + tile.width;
            self.x += self.speed;
          }
        }
      }

  }

  pub fn get_state(&self) -> (f32, f32, f32, f32) {
    (self.x, self.y, self.width, self.height)
  }

}
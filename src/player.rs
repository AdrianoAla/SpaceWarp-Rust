use macroquad::{prelude::*};

use crate::{utils::*};

pub struct Player {
  pub x: i32,
  pub y: i32,
  width: i32,
  height: i32,
  
  speed: i32,
  moving: bool,
  
  gravity: i32,
  jump: i32,
  jump_speed: i32,
  jump_height: i32,

  flip: bool,

  textures: [Texture2D; 3],

  
}
impl Player {
  pub fn new(x: i32, y: i32, width: i32, height: i32, player_images:[Texture2D; 3]) -> Player {
    Player {
      x,
      y,
      width,
      height,
      
      speed: 1,
      moving: false,
      
      gravity: 2,
      jump: 0,
      jump_height: 12,
      jump_speed: 2,

      flip: false,

      textures: player_images,
      
    }
  }
  
  pub fn draw(&self, frame: u64) {
    
    let mut params:DrawTextureParams = Default::default();
    params.flip_x = self.flip;
    
    let mut texture:Texture2D = *self.textures.clone().get(0).unwrap();
    
    if self.moving {
      texture = *self.textures.clone().get((frame%2) as usize).unwrap();
    }
    
    if self.jump > 0 {
      texture = *self.textures.clone().get(2).unwrap();
    }
    
    draw_texture_ex(texture, self.x as f32, self.y as f32, WHITE, params);
  }

  pub fn update(&mut self) {

      self.moving = false;

      // Left and right movement

      if is_key_down(KeyCode::Right)
         && get_collision(self.x+8, self.y+1) != 1
         && get_collision(self.x+8, self.y+7) != 1 
      {
        self.x += self.speed;
        self.flip = false;
        self.moving = true;
      }
      if is_key_down(KeyCode::Left)
         && get_collision(self.x, self.y+1) != 1
         && get_collision(self.x, self.y+7) != 1 
      {
        self.x -= self.speed;
        self.flip = true;
        self.moving = true;
      }

      // Jumping

      if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space)) && (get_collision(self.x+1, self.y+self.height) == 1 || get_collision(self.x+7, self.y+self.height) == 1)
      {
        self.jump = self.jump_height;
      }

      if get_collision(self.x+1, self.y-1) == 1 || get_collision(self.x+7, self.y-1) == 1
      {

        // Hit head on ceiling

        self.jump = 0;
      }

      // Gravity

      for _ in 0..self.gravity {
        if self.jump == 0 && get_collision(self.x+1, self.y+self.height) != 1 && get_collision(self.x+7, self.y+self.height) != 1
        {
          self.y += 1;
        }
      }

      // Jumping movement

      if self.jump > 0 {
        self.y -= self.jump_speed;
        self.jump -= 1;
      }


      // check if exited to the next screen

      if (self.x + self.width/2) > screen_size() {
        println!("Exit screen right");
      }
      if (self.x + self.width/2) < 0 {
        println!("Exit screen left");
      }
      if (self.y + self.height/2) > screen_size() {
        println!("Exit screen bottom");
      }
      if (self.y - self.height/2) < 0 {
        println!("Exit screen top");
      }

  }

  pub fn get_state(&self) -> (i32, i32, i32, i32, i32) {
    (self.x, self.y, self.width, self.height, self.jump)
  }

}
use macroquad::{prelude::*};

use crate::{utils::*, level::{get_level}};

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
  
  #[tokio::main]
  pub async fn new() -> Player {
    
    let spawn_point = get_level().lock().unwrap().get_spawn_location();

    let textures = get_textures().await;
  
    Player {
      x: spawn_point.0*8,
      y: spawn_point.1*8,
      width:8,
      height:8,
      
      speed: 1,
      moving: false,
      
      gravity: 2,
      jump: 0,
      jump_height: 12,
      jump_speed: 2,

      flip: false,

      textures,
      
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

      let mut unwrapped_level = get_level().lock().unwrap();

      if (self.x + self.width/2) > screen_size() {
        if !unwrapped_level.next(3) {
          self.x = screen_size()-self.width/2;
        } else {
          self.x = 0;
        }
      }
      if (self.x + self.width/2) < 0 {
        if !unwrapped_level.next(2) {
          self.x = -self.width/2;
        } else {
          self.x = screen_size()-self.width;
        }
      }
      if (self.y + self.height/2) > screen_size() {
        //CALLING HERE
        if !unwrapped_level.next(1) {
          self.y = screen_size()-self.height/2;
        } else {
          self.y = self.height;
        }
      }
      if (self.y - self.height/2) < 0 {
        if !unwrapped_level.next(0) {
          self.y = self.height/2;
        } else {
          self.y = screen_size()-self.height;
        }
      }

  }


  pub fn get_state(&self) -> (i32, i32, i32, i32, i32) {
    (self.x, self.y, self.width, self.height, self.jump)
  }

}

pub async fn get_textures() -> [Texture2D; 3] {
  // Load player textures

  let mut player_textures:[Texture2D; 3] = [load_texture("assets/player/player_1.png").await.unwrap(); 3];
    
  for i in 0..3 {
      let texture = load_texture(&format!("assets/player/player_{}.png", i+1)).await.unwrap();
      texture.set_filter(FilterMode::Nearest);
      player_textures[i] = texture;
  }
  
  player_textures
}
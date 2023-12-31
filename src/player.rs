use macroquad::{prelude::*, audio::{play_sound, PlaySoundParams}};

use crate::{utils::*, level::get_level};
use crate::tile::Tile;
use lazy_static::lazy_static;

#[derive(Clone, Copy)]
pub struct Player {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  
  speed: i32,
  moving: bool,
  
  gravity: i32,
  pub gravity_multiplier: i32,

  pub jump: i32,
  jump_speed: i32,
  jump_height: i32,

  pub flip: bool,

  textures: [Texture2D; 132],
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
      gravity_multiplier: 1,

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
    params.flip_y = self.gravity_multiplier < 0;
    
    let mut texture:Texture2D = *self.textures.clone().get((frame/2%44) as usize).unwrap();
    
    if self.moving {
      texture = *self.textures.clone().get((frame%44) as usize+44).unwrap();
    }
    
    if self.jump > 0 {
      texture = *self.textures.clone().get((((12-self.jump) as f32*3.5).round()) as usize + 88).unwrap();
    }
    
    draw_texture_ex(texture, self.x as f32, self.y as f32, WHITE, params);
  }

  pub fn update(&mut self) {

      self.moving = false;

      // Left and right movement

      if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
         && !is_solid(get_collision(self.x+8, self.y+1))
         && !is_solid(get_collision(self.x+8, self.y+7))
      {
        self.x += self.speed;
        self.flip = false;
        self.moving = true;
      }
      if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
         && !is_solid(get_collision(self.x, self.y+1))
         && !is_solid(get_collision(self.x, self.y+7))
      {
        self.x -= self.speed;
        self.flip = true;
        self.moving = true;
      }

      // Jumping

      if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space) || is_key_down(KeyCode::W))
      && (is_solid(get_collision(self.x+1, self.y+self.height)) || is_solid(get_collision(self.x+7, self.y+self.height)) || (self.y + self.height/2) >= screen_size())
      && !(is_solid(get_collision(self.x+1, self.y-1)) || is_solid(get_collision(self.x+7, self.y-1)))
      {
        self.jump = self.jump_height;
        play_sound(SOUND_JUMP.sound, PlaySoundParams {looped: false, volume: 0.15})
      }

      if is_solid(get_collision(self.x+1, self.y-1))
      || is_solid(get_collision(self.x+7, self.y-1))
      {
        // Hit head on ceiling
        self.jump = 0;
      }

      // Gravity

      for _ in 0..self.gravity {
        
        if self.jump == 0 && 
           !is_solid(get_collision(self.x+1, self.y+self.height)) &&
           !is_solid(get_collision(self.x+7, self.y+self.height))
        
        {
          self.y += 1 * self.gravity_multiplier;
        }
      
      }

      // Jumping movement

      if self.jump > 0 {
        self.y -= self.jump_speed;
        self.jump -= 1  * self.gravity_multiplier;
      }
      
      
      // Check if touching fire
      
      if is_kill(get_collision(self.x+1, self.y+1)) ||
         is_kill(get_collision(self.x+7, self.y+1)) ||
         is_kill(get_collision(self.x+1, self.y+7)) ||
         is_kill(get_collision(self.x+7, self.y+7)) {
        
        let mut level = get_level().lock().unwrap();
        let spawn_point = level.get_spawn_location();
        let original_state = level.original_state.clone();
        
        let mut tiles = [[Tile::new('⬜'); 16]; 16];
        for (y, row) in original_state.iter().enumerate() {
          for (x, tile) in row.iter().enumerate() {
            let owned = tile.clone();
            tiles[y][x] = owned;
          }
        }
        level.tiles = tiles;
        
        play_sound(DIE_SOUND.sound, PlaySoundParams { looped: false, volume: 1.5 });

        self.x = spawn_point.0*8;
        self.y = spawn_point.1*8;
        self.jump = 0;
        
      }
      
      let mut unwrapped_level = get_level().lock().unwrap();

      // check if exited to the next screen

      if (self.x + self.width/2) > screen_size() {

        // Exit from right
        if !unwrapped_level.next(3, self.clone()) {
          self.x = screen_size()-self.width/2;
        } else {
          self.x = 0;
        }
      }
      // Exit from left
      if (self.x + self.width/2) < 0 {
        if !unwrapped_level.next(2, self.clone()) {
          self.x = -self.width/2;
        } else {
          self.x = screen_size()-self.width;
        }
      }
      // Exit from bottom
      if (self.y + self.height/2) > screen_size() {
        if !unwrapped_level.next(1, self.clone()) {
          self.y = screen_size()-self.height/2;
        } else {
          self.y = self.height;
        }
      }

      // Exit from top
      if (self.y+self.height/2) < 0 {
        if !unwrapped_level.next(0, self.clone()) {
          self.y = -self.height/2;
          self.jump = 0;
        } else {
          self.y = screen_size()-self.height;
        }
      }

  }


  pub fn get_state(&self) -> (i32, i32, i32, i32, i32) {
    (self.x, self.y, self.width, self.height, self.jump)
  }

}

pub async fn get_textures() -> [Texture2D; 132] {
  // Load player textures

  let mut player_textures:[Texture2D; 132] = [load_texture("assets/player/idle_0.png").await.unwrap(); 132];
    
  for i in 0..44 {
      let texture = load_texture(&format!("assets/player/idle_{}.png", i)).await.unwrap();
      texture.set_filter(FilterMode::Nearest);
      player_textures[i] = texture;
  }

  for i in 44..88 {
    let texture = load_texture(&format!("assets/player/walk_{}.png", i-44)).await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    player_textures[i] = texture;
}

for i in 89..132 {
  let texture = load_texture(&format!("assets/player/jump_{}.png", i-89)).await.unwrap();
  texture.set_filter(FilterMode::Nearest);
  player_textures[i] = texture;
}
  
  player_textures
}

lazy_static! {
  static ref SOUND_JUMP: SoundLoader = SoundLoader::new(&format!("assets/sounds/jump.wav"));
  static ref DIE_SOUND: SoundLoader = SoundLoader::new(&format!("assets/sounds/explosion.wav"));
}
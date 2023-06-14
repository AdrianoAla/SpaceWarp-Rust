use std::f32::consts::PI;

use macroquad::{prelude::*};
use lazy_static::lazy_static;

pub struct Tile {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub color: Color,
  pub tile_type: char,
  textures: Vec<Texture2D>
} impl Tile {
  
  pub fn new(x: i32, y: i32, tile_type:char) -> Tile {

    let mut color:Color = WHITE;
    let mut textures = Vec::new();
    
    if tile_type == '⬛' {
      color = DARKGRAY;
    } else if tile_type == '⏴' || tile_type == '⏵' || tile_type == '⏷' || tile_type == '⏶' {
      textures.push(IMAGE_LOADER.get_texture());
    }


    Tile {
      x,
      y,
      width: 8,
      height: 8,
      color,
      tile_type,
      textures,
    }
  }

  pub fn is_fire(&self) -> bool {
    self.tile_type == '⏴' || self.tile_type == '⏵' || self.tile_type == '⏷' || self.tile_type == '⏶'
  }

  pub fn get_fire_rotation(&self) -> f32 {
    let mut rotation: f32 = 0.;
    if self.tile_type == '⏴' {
      rotation = PI*1.5;
    }
    if self.tile_type == '⏵' {
      rotation = PI/2.;
    }
    if self.tile_type == '⏷' {
      rotation = PI;
    }
    rotation
  }

  pub fn draw(&mut self) {
    if self.is_fire() {
      let params:DrawTextureParams = DrawTextureParams {  rotation: self.get_fire_rotation(), ..Default::default() };
      draw_texture_ex(*self.textures.get(0).unwrap(), self.x as f32, self.y as f32, WHITE, params);
    } else {
      draw_rectangle(self.x as f32, self.y as f32, self.width as f32, self.height as f32, self.color);
    }
  }

  pub fn get_state(&self) -> (i32, i32, i32, i32, char) {
    (self.x, self.y, self.width, self.height, self.tile_type)
  }
}

struct ImageLoader {
  pub texture: Texture2D,
}
impl ImageLoader {
  #[tokio::main]
  pub async fn new(path: &str) -> ImageLoader {
    let texture = load_texture(path).await.unwrap();
    ImageLoader {
      texture,
    }
  }

  pub fn get_texture(&self) -> Texture2D {
    self.texture
  }
}

lazy_static! {
  static ref IMAGE_LOADER: ImageLoader = ImageLoader::new("assets/fire.png");
}
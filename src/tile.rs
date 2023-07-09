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
  
  pub fn new(x: i32, y: i32, mut tile_type:char) -> Tile {

    let color:Color = WHITE;
    let mut textures = Vec::new();
    
    match tile_type {
      '⬆' => textures.push(IMAGE_WALL_1.get_texture()),
      '⬇' => textures.push(IMAGE_WALL_2.get_texture()),
      '⬅' => textures.push(IMAGE_WALL_3.get_texture()),
      '➡' => textures.push(IMAGE_WALL_4.get_texture()),
      '↖' => textures.push(IMAGE_WALL_5.get_texture()),
      '↗' => textures.push(IMAGE_WALL_6.get_texture()),
      '↙' => textures.push(IMAGE_WALL_7.get_texture()),
      '↘' => textures.push(IMAGE_WALL_8.get_texture()),
      '⏹' => textures.push(IMAGE_WALL_9.get_texture()),
      '⏪' => textures.push(IMAGE_WALL_10.get_texture()),
      '0' => textures.push(IMAGE_WALL_11.get_texture()),
      '⏩' => textures.push(IMAGE_WALL_12.get_texture()),
      '⏫' => textures.push(IMAGE_WALL_13.get_texture()),
      '1' => textures.push(IMAGE_WALL_14.get_texture()),
      '⏬' => textures.push(IMAGE_WALL_15.get_texture()),
      '⏺' => textures.push(IMAGE_WALL_16.get_texture()),
      '2' => textures.push(IMAGE_WALL_17.get_texture()),
      '3' => textures.push(IMAGE_WALL_18.get_texture()),
      '4' => textures.push(IMAGE_WALL_19.get_texture()),
      '5' => textures.push(IMAGE_WALL_20.get_texture()),
      '👈' | '👉' | '👇' | '👆' => textures.push(IMAGE_FIRE.get_texture()),
      _ => tile_type = '⬜',
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

  pub fn is_object(&self) -> bool {
    self.tile_type != '⬜'
  }
  
  pub fn is_fire(&self) -> bool {
    self.tile_type == '👈' || self.tile_type == '👉' || self.tile_type == '👇' || self.tile_type == '👆'
  }

  pub fn get_fire_rotation(&self) -> f32 {
    let mut rotation: f32 = 0.;
    if self.tile_type == '👈' {
      rotation = PI*1.5;
    }
    if self.tile_type == '👉' {
      rotation = PI/2.;
    }
    if self.tile_type == '👇' {
      rotation = PI;
    }
    rotation
  }

  pub fn draw(&mut self) {
    if self.is_object() {
      let params:DrawTextureParams = DrawTextureParams {  rotation: self.get_fire_rotation(), ..Default::default() };
      draw_texture_ex(*self.textures.get(0).unwrap(), self.x as f32, self.y as f32, self.color, params);
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
  static ref IMAGE_FIRE: ImageLoader = ImageLoader::new("assets/packs/metal/objects/fire.png");
  static ref IMAGE_WALL_1: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/top.png");
  static ref IMAGE_WALL_2: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/bottom.png");
  static ref IMAGE_WALL_3: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/left.png");
  static ref IMAGE_WALL_4: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/right.png");
  static ref IMAGE_WALL_5: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/top-left.png");
  static ref IMAGE_WALL_6: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/top-right.png");
  static ref IMAGE_WALL_7: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/bottom-left.png");
  static ref IMAGE_WALL_8: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/bottom-right.png");
  static ref IMAGE_WALL_9: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/square/center.png");
  static ref IMAGE_WALL_10: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/bottom/left.png");
  static ref IMAGE_WALL_11: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/bottom/center.png");
  static ref IMAGE_WALL_12: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/bottom/right.png");
  static ref IMAGE_WALL_13: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/top/top.png");
  static ref IMAGE_WALL_14: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/top/center.png");
  static ref IMAGE_WALL_15: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/top/bottom.png");
  static ref IMAGE_WALL_16: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/single.png");
  static ref IMAGE_WALL_17: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/corner/top-left.png");
  static ref IMAGE_WALL_18: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/corner/top-right.png");
  static ref IMAGE_WALL_19: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/corner/bottom-left.png");
  static ref IMAGE_WALL_20: ImageLoader = ImageLoader::new("assets/packs/metal/tiles/corner/bottom-right.png");
}
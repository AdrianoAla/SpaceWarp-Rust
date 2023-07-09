use std::f32::consts::PI;

use macroquad::{prelude::*};
use crate::utils::ImageLoader;
use lazy_static::lazy_static;


#[derive(Clone)]
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
      '🟥' => textures.push(BOTTOM_DOOR_RED.get_texture()),
      '❤' => textures.push(KEY_RED.get_texture()),
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

  pub fn is_door(&self) -> bool {
    self.tile_type == '🟥'
  }

  pub fn draw(&mut self) {
    if self.is_object() {
      if self.is_door() {
        draw_texture_ex(TOP_DOOR_RED.get_texture(), self.x as f32, self.y as f32 - 8.0, self.color, DrawTextureParams::default());  
      }
      let params:DrawTextureParams = DrawTextureParams {  rotation: self.get_fire_rotation(), ..Default::default() };
      draw_texture_ex(*self.textures.get(0).unwrap(), self.x as f32, self.y as f32, self.color, params);
    } else {
      draw_rectangle(self.x as f32, self.y as f32, self.width as f32, self.height as f32, self.color);
    }
  }

}

lazy_static! {
  static ref PACK: String = {
    let pack = "metal";
    pack.to_owned()
  };

  static ref IMAGE_FIRE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/fire.png", *PACK));
  static ref IMAGE_WALL_1: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/top.png", *PACK));
  static ref IMAGE_WALL_2: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/bottom.png", *PACK));
  static ref IMAGE_WALL_3: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/left.png", *PACK));
  static ref IMAGE_WALL_4: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/right.png", *PACK));
  static ref IMAGE_WALL_5: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/top-left.png", *PACK));
  static ref IMAGE_WALL_6: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/top-right.png", *PACK));
  static ref IMAGE_WALL_7: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/bottom-left.png", *PACK));
  static ref IMAGE_WALL_8: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/bottom-right.png", *PACK));
  static ref IMAGE_WALL_9: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/center.png", *PACK));
  static ref IMAGE_WALL_10: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/bottom/left.png", *PACK));
  static ref IMAGE_WALL_11: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/bottom/center.png", *PACK));
  static ref IMAGE_WALL_12: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/bottom/right.png", *PACK));
  static ref IMAGE_WALL_13: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/top/top.png", *PACK));
  static ref IMAGE_WALL_14: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/top/center.png", *PACK));
  static ref IMAGE_WALL_15: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/top/bottom.png", *PACK));
  static ref IMAGE_WALL_16: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/single.png", *PACK));
  static ref IMAGE_WALL_17: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/top-left.png", *PACK));
  static ref IMAGE_WALL_18: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/top-right.png", *PACK));
  static ref IMAGE_WALL_19: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/bottom-left.png", *PACK));
  static ref IMAGE_WALL_20: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/bottom-right.png", *PACK));
  static ref BOTTOM_DOOR_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/door/bottom.png", *PACK));
  static ref TOP_DOOR_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/door/top.png", *PACK));
  static ref KEY_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/key.png", *PACK));
}
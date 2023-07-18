use std::f32::consts::PI;

use macroquad::prelude::*;
use crate::utils::{ImageLoader, PACK};
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq)]
pub enum ObjectColor {
  Red,
  Blue,
  Yellow,
  None,
}

#[derive(Clone, Copy)]
pub struct Tile {

  pub ox: i32,
  pub oy: i32,

  pub width: i32,
  pub height: i32,
  
  pub tile_type: char,
  texture: Option<Texture2D>,
  pub collidable: bool,
  pub timer: i32,
  pub anim_timer: i32,
  pub visible: bool,
  pub locked: bool,
} impl Tile {
  
  pub fn new(mut tile_type:char) -> Tile {

    let mut texture = None;
    
    match tile_type {
      '⬆' => texture = Some(IMAGE_WALL_1.texture),
      '⬇' => texture = Some(IMAGE_WALL_2.texture),
      '⬅' => texture = Some(IMAGE_WALL_3.texture),
      '➡' => texture = Some(IMAGE_WALL_4.texture),
      '↖' => texture = Some(IMAGE_WALL_5.texture),
      '↗' => texture = Some(IMAGE_WALL_6.texture),
      '↙' => texture = Some(IMAGE_WALL_7.texture),
      '↘' => texture = Some(IMAGE_WALL_8.texture),
      '⏹' => texture = Some(IMAGE_WALL_9.texture),
      '⏪' => texture = Some(IMAGE_WALL_10.texture),
      '0' => texture = Some(IMAGE_WALL_11.texture),
      '⏩' => texture = Some(IMAGE_WALL_12.texture),
      '⏫' => texture = Some(IMAGE_WALL_13.texture),
      '1' => texture = Some(IMAGE_WALL_14.texture),
      '⏬' => texture = Some(IMAGE_WALL_15.texture),
      '⏺' => texture = Some(IMAGE_WALL_16.texture),
      '2' => texture = Some(IMAGE_WALL_17.texture),
      '3' => texture = Some(IMAGE_WALL_18.texture),
      '4' => texture = Some(IMAGE_WALL_19.texture),
      '5' => texture = Some(IMAGE_WALL_20.texture),
      '👈' | '👉' | '👇' | '👆' => texture = Some(IMAGE_FIRE.texture),
      
      '🟥' => texture = Some(BOTTOM_DOOR_RED.texture),
      '🟦' => texture = Some(BOTTOM_DOOR_BLUE.texture),
      '🟨' => texture = Some(BOTTOM_DOOR_YELLOW.texture),
      
      '❤' => texture = Some(KEY_RED.texture),
      '💙' => texture = Some(KEY_BLUE.texture),
      '💛' => texture = Some(KEY_YELLOW.texture),

      '🔴' => texture = Some(BUTTON_RED.texture),
      '🔵' => texture = Some(BUTTON_BLUE.texture),
      '🟡' => texture = Some(BUTTON_YELLOW.texture),
      
      '🚀' => texture = Some(BUTTON_YELLOW.texture),
      
      _ => tile_type = '⬜',
    }

    Tile {
      ox: 0,
      oy: 0,
      
      width: 8,
      height: 8,
      
      tile_type,
      
      texture,
      
      collidable: true,
      
      timer: 0,
      anim_timer:0 ,
      
      visible: true,
      locked: false,
    }
  }

  pub fn is_object(&self) -> bool {
    self.tile_type != '⬜'
  }

  pub fn is_key(&self) -> ObjectColor {
    if self.tile_type == '❤' {
      return ObjectColor::Red;
    }
    else if self.tile_type == '💙' {
      return ObjectColor::Blue;
    }
    else if self.tile_type == '💛' {
      return ObjectColor::Yellow;
    }
    ObjectColor::None
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

  pub fn is_button(&self) -> ObjectColor {
    if self.tile_type == '🔴' {
      return ObjectColor::Red;
    }
    else if self.tile_type == '🔵' {
      return ObjectColor::Blue;
    }
    else if self.tile_type == '🟡' {
      return ObjectColor::Yellow;
    }
    ObjectColor::None
  }

  pub fn is_door(&self) -> ObjectColor {
    if self.tile_type == '🟥' {
      return ObjectColor::Red;
    }
    else if self.tile_type == '🟦' {
      return ObjectColor::Blue;
    }
    else if self.tile_type == '🟨' {
      return ObjectColor::Yellow;
    }
    ObjectColor::None
  }

  pub fn update(&mut self, frame: u64) {
   
    match self.is_key() {
      ObjectColor::None => {}
      _ => {
        if frame % 12 == 0 {
          self.oy += 1;
          self.oy %= 2;
        }
      }
    }

    if self.timer > 0 {
      self.timer -= 1;
      
      if self.timer == 0 {
        self.collidable = true;
        self.visible = true;
        match self.is_door() {
          ObjectColor::None => {},
          _ => {self.anim_timer = 8;}
        }

        match self.is_button() {
          ObjectColor::None => {},
          _ => {self.anim_timer = 8;}
        }
      }
    
    }

    if self.anim_timer > 0 {
      
      self.anim_timer -= 1;
      
      if self.collidable {
        self.oy -= 1;
      } else {
        self.oy += 1;
        if self.anim_timer == 0 {
          self.visible = false;
        }
      }
    
    }
  }

  pub fn draw(&mut self, x: i32, y: i32) {

    let x = x * 8;
    let y = y * 8;
    
    if !self.visible {return;}

    if self.is_object() {
      
      match self.is_door() {
        ObjectColor::Red => {
          draw_texture_ex(TOP_DOOR_RED.texture, x as f32, (y - self.oy) as f32 - 8.0, WHITE, DrawTextureParams::default());
        },
        ObjectColor::Blue => {
            draw_texture_ex(TOP_DOOR_BLUE.texture, x as f32, (y - self.oy) as f32 - 8.0, WHITE, DrawTextureParams::default());
        },
        ObjectColor::Yellow => {
          draw_texture_ex(TOP_DOOR_YELLOW.texture, x as f32, (y - self.oy) as f32 - 8.0, WHITE, DrawTextureParams::default());
        },
        ObjectColor::None => {},
      }
      
      let params:DrawTextureParams = DrawTextureParams {  rotation: self.get_fire_rotation(), ..Default::default() };
      
      draw_texture_ex(self.texture.expect("msg"), x as f32, (y + self.oy) as f32, WHITE, params);

    }
  }

}

lazy_static! {
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
  
  static ref BOTTOM_DOOR_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/door/bottom.png", *PACK));
  static ref TOP_DOOR_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/door/top.png", *PACK));
  
  static ref BOTTOM_DOOR_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/door/bottom.png", *PACK));
  static ref TOP_DOOR_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/door/top.png", *PACK));
  
  static ref KEY_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/key.png", *PACK));
  static ref KEY_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/key.png", *PACK));
  static ref KEY_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/key.png", *PACK));

  static ref BUTTON_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/button.png", *PACK));
  static ref BUTTON_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/button.png", *PACK));
  static ref BUTTON_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/button.png", *PACK));
}
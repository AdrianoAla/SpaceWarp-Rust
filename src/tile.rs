use std::f32::consts::PI;

use macroquad::{prelude::*};
use crate::utils::ImageLoader;
use lazy_static::lazy_static;

#[derive(Clone, Copy)]
pub enum ObjectColor {
  Red,
  Blue,
  Yellow,
  None,
}

#[derive(Clone)]
pub struct Tile {
  pub x: i32,
  pub y: i32,
  pub oy: i32,
  pub width: i32,
  pub height: i32,
  pub color: Color,
  pub tile_type: char,
  textures: Vec<Texture2D>,
  pub collidable: bool,
  pub timer: i32,
  pub anim_timer: i32,
  pub visible: bool,
  pub locked: bool,
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
      '🟦' => textures.push(BOTTOM_DOOR_BLUE.get_texture()),
      '🟨' => textures.push(BOTTOM_DOOR_YELLOW.get_texture()),
      
      '❤' => textures.push(KEY_RED.get_texture()),
      '💙' => textures.push(KEY_BLUE.get_texture()),
      '💛' => textures.push(KEY_YELLOW.get_texture()),

      '🔴' => textures.push(BUTTON_RED.get_texture()),
      '🔵' => textures.push(BUTTON_BLUE.get_texture()),
      '🟡' => textures.push(BUTTON_YELLOW.get_texture()),
      
      _ => tile_type = '⬜',
    }

    Tile {
      x,
      y,
      oy: y,
      width: 8,
      height: 8,
      color,
      tile_type,
      textures,
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

  pub fn is_door(&self, color: ObjectColor) -> bool {
    match color {
      ObjectColor::Red => self.tile_type == '🟥',
      ObjectColor::Blue => self.tile_type == '🟦',
      ObjectColor::Yellow => self.tile_type == '🟨',
      ObjectColor::None => self.tile_type == '🟥' || self.tile_type == '🟦' || self.tile_type == '🟨'
    }
  }

  pub fn update(&mut self) {
    if self.timer > 0 {
      self.timer -= 1;
      println!("{}, {}", self.tile_type, self.timer);
      
      if self.timer == 0 {
        println!("timer over");
        self.collidable = true;
        self.visible = true;
        if self.is_door(ObjectColor::None) {
          self.anim_timer = 8;
        }
      }
    
    }

    if self.anim_timer > 0 {
      if self.anim_timer == 8 {
        if !self.collidable {
          self.y = self.oy;
        } else {
          self.y = self.oy + 8;
        }
      }
      self.anim_timer -= 1;
      if self.collidable {
        self.y -= 1;
      } else {
        self.y += 1;
        if self.anim_timer == 0 {
          self.visible = false;
        }
      }
    }
  }

  pub fn draw(&mut self) {
    
    if !self.visible {return;}

    match self.is_button() {
      ObjectColor::None => {},
      _ => {if !self.collidable {return;}}
    }

    if self.is_object() {
      if self.is_door(ObjectColor::Red) {
        if !self.collidable {
          draw_texture_ex(TOP_DOOR_RED.get_texture(), self.x as f32, (self.y - (8-self.anim_timer)*2) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
        else if self.anim_timer != 0 {
          draw_texture_ex(TOP_DOOR_RED.get_texture(), self.x as f32, (self.y - (self.anim_timer)*2) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
        else {
          draw_texture_ex(TOP_DOOR_RED.get_texture(), self.x as f32, (self.y) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
      }
      if self.is_door(ObjectColor::Blue) {
        if !self.collidable {
          draw_texture_ex(TOP_DOOR_BLUE.get_texture(), self.x as f32, (self.y - (8-self.anim_timer)*2) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
        else if self.anim_timer != 0 {
          draw_texture_ex(TOP_DOOR_BLUE.get_texture(), self.x as f32, (self.y - (self.anim_timer)*2) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
        else {
          draw_texture_ex(TOP_DOOR_BLUE.get_texture(), self.x as f32, (self.y) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
      }
      if self.is_door(ObjectColor::Yellow) {
        if !self.collidable {
          draw_texture_ex(TOP_DOOR_YELLOW.get_texture(), self.x as f32, (self.y - (8-self.anim_timer)*2) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
        else if self.anim_timer != 0 {
          draw_texture_ex(TOP_DOOR_YELLOW.get_texture(), self.x as f32, (self.y - (self.anim_timer)*2) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
        else {
          draw_texture_ex(TOP_DOOR_YELLOW.get_texture(), self.x as f32, (self.y) as f32 - 8.0, self.color, DrawTextureParams::default());
        }
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
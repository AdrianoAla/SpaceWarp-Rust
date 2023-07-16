use std::f32::consts::PI;
use macroquad::prelude::*;
use macroquad_text::*;
use lazy_static::lazy_static;
use crate::utils::*;

pub enum MenuEvents {
  NoOp,
  GoToGame,
  GoToEditor,
  GoToSettings,
  Quit,
}

pub struct Menu {
  ship_x: f64,
  ship_y: f64,
  ship_angle: f64,
  ship_dx: f64,
  ship_dy: f64,
  animating: bool,
  anim_frames: i32,
  pub options: Vec<String>,
  selected_option: i32,
} impl Menu {
  
  pub fn new() -> Menu {
    Menu {
      ship_x: 0.0,
      ship_y: 0.0,
      ship_angle: (PI as f64/4.0)*3.0,
      ship_dx: 0.1,
      ship_dy: 0.1,
      animating: false,
      anim_frames: 0,
      options: vec![String::from("Play"), String::from("Editor"), String::from("Settings"), String::from("Quit")],
      selected_option: 0,
    }
  }

  pub fn render(&mut self, fonts: &mut Fonts) {
    clear_background(BLACK);
                    
    draw_texture(BACKGROUND_TEXTURE.texture, 0.0, 0.0, WHITE);

    draw_texture_ex(ROCKET_TEXTURE.texture, self.ship_x as f32, self.ship_y as f32, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(12.5, 16.0)), rotation: self.ship_angle as f32, ..Default::default() });

    draw_rectangle(10.0, 5.0, screen_size() as f32 - 20.0, screen_size() as f32 - 10.0, Color::from_rgba(0, 0, 0, 200));
    
    draw_texture_ex(SPACEWARP_TEXTURE.texture, 23.0, 5.0, WHITE, DrawTextureParams {dest_size: Some(Vec2::new(80.0,35.0)), ..Default::default()});

        //fonts.draw_text(&format!("SpaceWarp"), (screen_size()/2) as f32 - (fonts.measure_text(&format!("SpaceWarp"), 8).width/2.0), 15.0, 8, WHITE);

    for (index, option) in self.options.iter().enumerate() {
        let mut color = WHITE;
        if index as i32 == self.selected_option {
            color = YELLOW;
        }
        fonts.draw_text(&format!("{option}"), (screen_size()/2) as f32 - (fonts.measure_text(&format!("{option}"), 8).width/2.0), (index*20+45) as f32, 8, color);
    }

    if self.animating {
        draw_rectangle(0.0, 0.0, screen_size() as f32, screen_size() as f32, Color::from_rgba(0, 0, 0, 255-(self.anim_frames*5) as u8));
    }
  }

  pub fn update(&mut self) {
    self.ship_x += self.ship_dx;
    self.ship_y += self.ship_dy;

    if self.ship_x >= 115.0 {
      self.ship_dx = -0.1;
      self.ship_dy = -0.1;
      self.ship_angle = (PI as f64/4.0)*7.0;
    } 

    if self.ship_x <= 0.0 {
      self.ship_dx = 0.1;
      self.ship_dy = 0.1;
      self.ship_angle = (PI as f64/4.0)*3.0;
    } 

    if self.anim_frames > 0 {
      self.anim_frames -= 1;
    }

    
  }


  pub fn get_action(&mut self) -> MenuEvents {

    if is_key_pressed(KeyCode::Up) {
      self.selected_option -= 1;
      if self.selected_option < 0 {
        self.selected_option = (self.options.len()-1) as i32;
      }
    }

    if is_key_pressed(KeyCode::Down) {
      self.selected_option += 1;
      self.selected_option %= self.options.len() as i32;
    }

    if self.anim_frames == 0 && self.animating {
      return MenuEvents::GoToGame;
    }

    if is_key_pressed(KeyCode::Enter) {
      if self.options[self.selected_option as usize] == "Play" && !self.animating {
          self.anim_frames = 51;
          self.animating = true;
      } else if self.options[self.selected_option as usize] == "Settings" {
        return MenuEvents::GoToSettings;
      } 
      else if self.options[self.selected_option as usize] == "Editor" {
          return MenuEvents::GoToEditor;
      }
      else if self.options[self.selected_option as usize] == "Quit" {
          return MenuEvents::Quit;
      }
    }
    return MenuEvents::NoOp;
  }

}

lazy_static! {

  static ref BACKGROUND_TEXTURE: ImageLoader = ImageLoader::new("assets/background.png");
  static ref SPACEWARP_TEXTURE: ImageLoader = ImageLoader::new("assets/spacewarp.png");
  static ref ROCKET_TEXTURE: ImageLoader = ImageLoader::new("assets/rocket.png");

}
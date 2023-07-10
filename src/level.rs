use std::fs::File;
use std::io::prelude::*;
use crate::tile::{Tile, ObjectColor};
use macroquad::prelude::*;

use std::sync::Mutex;
use lazy_static::lazy_static;
 
 #[derive(Clone)]
pub struct Level {
  pub current_file: String,
  pub tiles: Vec<Tile>,
  pub next_levels: Vec<String>,
  pub spawn_point: (i32, i32),
  pub original_state: Vec<Tile>,
  pub previous_levels: Vec<Level>,
} 

impl Level {

  pub fn new(filename:&str) -> Level {load_from_file(filename)}

  pub fn draw(&mut self) {
    for tile in &mut self.tiles {
      if tile.is_door(ObjectColor::None) {
        tile.draw();
      }
    }

    for tile in &mut self.tiles {
      if !tile.is_door(ObjectColor::None) {
        tile.draw();
      }
    }
  }

  pub fn update(&mut self) {
    for tile in self.tiles.iter_mut() {
      tile.update();
    }
  }
  
  pub fn next(&mut self, dir:i32) -> bool  {

    println!("Init Level Transition");

    let mut add = true;
    for level in self.previous_levels.iter() {
      if level.current_file == self.current_file {
        add = false;
      }
    } 
    if add {self.previous_levels.push(self.clone());}
  
    let new_file = self.next_levels.get(dir as usize).unwrap();
    if new_file == "-1" {return false;}
    
    let mut new_level: Option<Level> = None;
    for level in self.previous_levels.iter_mut() {
      println!("{} || {}", level.current_file, format!("level_{new_file}.sw"));
      if level.current_file == format!("level_{new_file}.sw") {
        new_level = Some(level.clone());
      }
    }

    match new_level {
      None => {
        new_level = Some(load_from_file(format!("level_{new_file}.sw").as_str()));
      }
      _ => {}
    }

    let new_level = new_level.unwrap();

    self.spawn_point = (new_level.spawn_point.0,new_level.spawn_point.1);
    
    
    
    self.current_file = format!("level_{new_file}.sw");
    self.next_levels = new_level.next_levels;
    self.tiles = new_level.tiles;
    self.original_state = new_level.original_state;

    return true;
    
  }

  pub fn get_spawn_location(&self) -> (i32, i32) {
    self.spawn_point
  }
}

pub fn load_from_file(filename:&str) -> Level {
  let file_name = filename;
    let mut file = File::open(format!("levels/{file_name}")).unwrap();
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).unwrap();
    
    // filter out \u{fe0f} and \u{20e3} (aka: extra emoji characters)
    
    let filtered_input: String = contents
        .chars()
        .filter(|&c| c != '\u{fe0f}' && c != '\u{20e3}')
        .collect();
    
    // separate into vec by newline
    
    let lines: Vec<&str> = filtered_input.lines().collect();
    
    // create new tile for each character in each line

    let mut tiles:Vec<Tile> = Vec::new();
    let mut original_state: Vec<Tile> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c != '⬜' {
          tiles.push(Tile::new((x * 8) as i32, (y * 8) as i32, c));
          original_state.push(Tile::new((x * 8) as i32, (y * 8) as i32, c));
        }
      }
    }
    

    let mut next_levels:Vec<String> = Vec::new();
    
    for (_y, line) in lines.iter().enumerate().skip(16).take(4) {
      next_levels.push(String::from(*line));
    }
    
    let x = (lines.get(20)).expect("error").parse::<i32>().expect("Error");
    let y = (lines.get(21)).expect("error").parse::<i32>().expect("Error");
    
    Level {
      current_file: String::from(filename),
      tiles,
      next_levels,
      spawn_point: (x,y),
      original_state,
      previous_levels: Vec::new()
    }
}

lazy_static! {
  pub static ref LEVEL:Mutex<Level> = Mutex::new(Level::new("level_1.sw"));
}

pub fn get_level() -> &'static Mutex<Level>  {
  &LEVEL
}





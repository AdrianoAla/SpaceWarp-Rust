use std::fs::File;
use std::io::prelude::*;
use crate::tile::Tile;
use macroquad::prelude::*;

use std::sync::Mutex;
use lazy_static::lazy_static;
 
pub struct Level {
  pub current_file: String,
  pub tiles: Vec<Tile>,
  pub next_levels: Vec<String>,
  pub spawn_point: (i32, i32),
  pub next_levels_items: Vec<Level>,
} 

impl Level {

  pub fn new(filename:&str) -> Level {load_from_file(filename)}

  pub fn draw(&mut self) {
    for tile in &mut self.tiles {
      tile.draw();
    }
  }
  
  pub fn next(&mut self, dir:i32) -> bool  {
  
    let new_file = self.next_levels.get(dir as usize).unwrap();
    if new_file == "-1" {return false;}
    
    let file_name = format!("level_{}.sw", new_file);
    
    let new_level = load_from_file(&file_name);

    self.spawn_point = (new_level.spawn_point.0,new_level.spawn_point.1);
    
    
    
    self.current_file = String::from(new_file);
    self.next_levels = new_level.next_levels;
    self.tiles = new_level.tiles;
    self.next_levels_items = new_level.next_levels_items;

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

    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c != '⬜' {
          tiles.push(Tile::new((x * 8) as i32, (y * 8) as i32, c));
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
      next_levels_items: Vec::new(),
    }
}

lazy_static! {
  pub static ref LEVEL:Mutex<Level> = Mutex::new(Level::new("level_1.sw"));
}

pub fn get_level() -> &'static Mutex<Level>  {
  &LEVEL
}





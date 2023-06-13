use std::fs::File;
use std::io::prelude::*;
use crate::tile::Tile;
use macroquad::prelude::*;

use lazy_static::lazy_static;

 
pub struct Level {
  pub tiles: Vec<Tile>,
  pub width: i32,
  pub height: i32,
} impl Level {

  pub fn new(filename:&str) -> Level {

    let file_name = filename;
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).unwrap();
    
    // separate into vec by newline
    
    let lines: Vec<&str> = contents.split("\n").collect();
    
    // create new tile for each character in each line

    let mut tiles:Vec<Tile> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          tiles.push(Tile::new((x * 8) as i32, (y * 8) as i32, BLUE));
        }
      }
    }


    Level {
      tiles,
      width: 16,
      height: 16,
    }
  }

  pub fn draw(&self) {
    for tile in &self.tiles {
      tile.draw();
    }
  }

  pub fn get_tiles(&self) -> &Vec<Tile> {
    &self.tiles
  }
}

lazy_static! {
  pub static ref LEVEL: Level = Level::new("level.sw");
}

pub fn get_level() -> &'static Level {
  &LEVEL
}
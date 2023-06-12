use macroquad::{prelude::*};
use macroquad_canvas_2d::*;

mod player;
use player::Player;

mod utils;
use utils::*;

mod tile;
use tile::Tile;

#[macroquad::main("SpaceWarp: Definitive Edition")]
async fn main() {
    
    let canvas = Canvas2D::new(screen_size(), screen_size());

    // Get the tiles

    let tiles: Vec<Tile> = get_tiles();
    
    // Create a new player
    
    let mut player = Player::new(0.0, 0.0, 50.0, 50.0);

    
    
    let mut accumulator: f32 = 0.0;
    let mut frames: u32 = 0;
    loop {
        
        canvas.set_camera();
        {
            let mut delta_frame_time = get_frame_time();
            
            if (delta_frame_time - 1.0/120.0).abs() < 0.0002 {
                delta_frame_time = 1.0/120.0;
            }
            if (delta_frame_time - 1.0/60.0).abs() < 0.0002 {
                delta_frame_time = 1.0/60.0;
            }
            if (delta_frame_time - 1.0/30.0).abs() < 0.0002 {
                delta_frame_time = 1.0/30.0;
            } accumulator += delta_frame_time;
            
            while accumulator >= 1.0 / 60.0 {
                player.update();
                frames += 1;
                println!("{}", format!("Accumulator: {} Delta Frame Time: {}", accumulator, delta_frame_time));
                accumulator -= 1.0 / 60.0;
            }
           
            
            clear_background(LIGHTGRAY);

            // Debug text

            draw_debug_text(&player, &tiles, frames);

            // Draw the player

            player.draw();

            // Draw all tiles

            for tile in &tiles {
                tile.draw();
            }

            // Break the loop if the escape key is pressed

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            
        }
        
        set_default_camera();
        canvas.draw_to_screen();

        next_frame().await
    }
}
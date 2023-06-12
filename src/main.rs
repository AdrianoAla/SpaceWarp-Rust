use macroquad::{prelude::*};
use macroquad_canvas_2d::*;

mod player;
use player::Player;

mod utils;
use utils::*;

mod tile;
use tile::Tile;

#[macroquad::main("BasicShapes")]
async fn main() {
    
    let canvas = Canvas2D::new(screen_size(), screen_size());

    // Get the tiles

    let tiles: Vec<Tile> = get_tiles();
    
    // Create a new player
    
    let mut player = Player::new(0.0, 0.0, 50.0, 50.0);

    
    loop {
        
        canvas.set_camera();
        {
            
            clear_background(LIGHTGRAY);

            // Debug text

            draw_text(&format!("x: {} y: {}", player.get_state().0, player.get_state().1), 0.0, 20.0, 30.0, BLACK);

            // Draw and update the player

            player.draw();
            player.update();

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
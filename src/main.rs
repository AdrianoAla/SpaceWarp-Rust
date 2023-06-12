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

    
    
    loop {
        
        canvas.set_camera();
        {
            
            clear_background(LIGHTGRAY);

            // Debug text

            draw_text(&format!("X: {} Y: {}", player.get_state().0, player.get_state().1),     0.0, 20.0, 30.0, BLACK);
            draw_text(&format!("Can Jump: {}", player.get_state().4),                          0.0, 45.0, 30.0, BLACK);
            draw_text(&format!("Tiles Loaded: {}", tiles.len()),                               0.0, 70.0, 30.0, BLACK);
            draw_text(&format!("FPS: {}", get_fps()),                               0.0, 95.0, 30.0, BLACK);
            draw_text(&format!("Frametime: {}", get_frame_time()),                               0.0, 120.0, 30.0, BLACK);

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
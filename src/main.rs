use macroquad::{prelude::*};
use macroquad_text::*;

mod canvas;
use canvas::Canvas2D;

mod player;
use player::Player;

mod utils;
use utils::*;

mod tile;

mod level;
use level::get_level;

const FONT: &[u8] = include_bytes!("../assets/font.ttf");
const TARGET_FPS: u32 = 30;

#[macroquad::main("SpaceWarp: Definitive Edition")]
async fn main() {

    // Load a font

    let mut fonts = Fonts::new(ScalingMode::Linear);
    fonts.load_font_from_bytes(FONT).unwrap();

    // Load player textures

    let mut player_textures:[Texture2D; 3] = [load_texture("assets/player/player_1.png").await.unwrap(); 3];
    
    for i in 0..3 {
        let texture = load_texture(&format!("assets/player/player_{}.png", i+1)).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        player_textures[i] = texture;
    }

    // Create a new canvas
    
    let canvas = Canvas2D::new(screen_size() as f32, screen_size() as f32);
    
    // Create a new player
    
    let mut player = Player::new(0, 0, 8, 8, player_textures);

    
    
    let mut accumulator: f32 = 0.0;
    let mut frame: u64 = 0;
    
    loop {
        
        set_camera(&canvas.camera);
        {

            // Limit updates to 30 TPS 

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
            
            while accumulator >= 1.0 / TARGET_FPS as f32 {
                
                // RUN ALL UPDATE FUNCTIONS HERE

                player.update();
                
                frame += 1;
            
            
                accumulator -= 1.0 / TARGET_FPS as f32;
            }
           
            
            clear_background(BLACK);

           
            // Draw the player

            player.draw(frame);

            // Draw level

            get_level().draw();

            // Debug text

            draw_debug_text(&player, &fonts);


            // Break the loop if the escape key is pressed

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            
        }
        
        set_default_camera();
        canvas.draw();

        next_frame().await
    }
}
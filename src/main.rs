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
use level::*;

const FONT: &[u8] = include_bytes!("../assets/font.ttf");

#[macroquad::main("SpaceWarp: Definitive Edition")]
async fn main() {

    // Create a level
    
    let level = get_level();
    
    // Create a player

    let mut player:Player = Player::new();

    let mut target_fps: u32 = 30;
    
    // Load background texture

    let background_texture = load_texture("assets/background.png").await.unwrap();
    
    // Load a font

    let mut fonts = Fonts::new(ScalingMode::Linear);
    fonts.load_font_from_bytes(FONT).unwrap();

    // Create a canvas
    
    let canvas = Canvas2D::new(screen_size() as f32, screen_size() as f32);

    // Debug

    let mut accumulator: f32 = 0.0;
    let mut frame: u64 = 0;
    let mut allow_update:bool = false;
    let mut allow_debug:bool = false;
    
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
            
            while accumulator >= 1.0 / target_fps as f32 {
                
                // RUN ALL UPDATE FUNCTIONS HERE
                if allow_update || !allow_debug {
                    
                    allow_update = false;
                    // update the player

                    player.update();

                    frame += 1;
                
                }
                accumulator -= 1.0 / target_fps as f32;
            }
           
            
            clear_background(BLACK);

            draw_texture(background_texture, 0.0, 0.0, WHITE);

            // Draw the player
            
            player.draw(frame);

            // Draw level

            level.lock().unwrap().draw();

            // Debug text

            if allow_debug {
                draw_debug_text(&fonts, &target_fps, &player);
            }


            // Break the loop if the escape key is pressed

            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            
            // Debug features
            
            if is_key_pressed(KeyCode::LeftBracket) {
                target_fps -= 5;
            }
            if is_key_pressed(KeyCode::RightBracket) {
                target_fps += 5;
            }
            
            if is_key_pressed(KeyCode::F1) {
                allow_debug = !allow_debug;
            }
            if is_key_pressed(KeyCode::Enter) {
                allow_update = true
            }

            
        }
        
        set_default_camera();
        canvas.draw();

        next_frame().await
    }
}
use std::f32::consts::PI;

use macroquad::{prelude::*, audio::{play_sound, PlaySoundParams}};
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

use lazy_static::lazy_static;

const FONT: &[u8] = include_bytes!("../assets/font.ttf");

enum GameStates {
    Menu,
    Game,
    Win,
    Pause,
}

#[macroquad::main("SpaceWarp: Definitive Edition")]
async fn main() {

    // Create a canvas
    
    let canvas = Canvas2D::new(screen_size() as f32, screen_size() as f32);

    // Load a font

    let mut fonts = Fonts::new(ScalingMode::Linear);
    fonts.load_font_from_bytes(FONT).unwrap();

    set_camera(&canvas.camera);
    {
        fonts.draw_text(&format!("Loading"), (screen_size()/2) as f32 - (fonts.measure_text(&format!("Loading"), 8).width/2.0), 15.0, 8, WHITE);
    }
        
    set_default_camera();
    canvas.draw();

    next_frame().await;

    // Create a level
    
    let level = get_level();
    
    // Create a player

    let mut player:Player = Player::new();

    let mut game_state: GameStates = GameStates::Menu;

    let mut target_fps: u32 = 30;
    
    // Load background texture

    let background_texture = load_texture("assets/background.png").await.unwrap();

    let ship = load_texture("assets/rocket.png").await.unwrap();
    let mut ship_x = 0.0;
    let mut ship_y = 0.0;
    let mut ship_angle: f64 = (PI as f64/4.0)*3.0;
    let mut ship_dx = 0.1;
    let mut ship_dy = 0.1;

    // Debug

    let mut accumulator: f32 = 0.0;
    let mut frame: u64 = 0;
    let mut allow_update:bool = false;
    let mut allow_debug:bool = false;

    let mut anim_frames = 0;
    let mut animating = false;

    let mut selected_option: i32 = 0;

    let options = vec!["Play", "Settings", "Quit"];
    
    loop {
        
        set_camera(&canvas.camera);
        {

            
           
            
            match game_state {

                GameStates::Menu => {

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
                        
                        ship_x += ship_dx;
                        ship_y += ship_dy;

                        if ship_x >= 115.0 {
                            ship_dx = -0.1;
                            ship_dy = -0.1;
                            ship_angle = (PI as f64/4.0)*7.0;
                        } 

                        if ship_x <= 0.0 {
                            ship_dx = 0.1;
                            ship_dy = 0.1;
                            ship_angle = (PI as f64/4.0)*3.0;
                        } 
                        accumulator -= 1.0 / target_fps as f32;

                        if anim_frames > 0 {
                            anim_frames -= 1;
                            if anim_frames == 0 {
                                game_state = GameStates::Game;
                                frame = 0;
                                play_sound(BGM.sound, PlaySoundParams {looped:true, volume: 1.0});
                            }
                        }
                    }

                    clear_background(BLACK);
                    
                    draw_texture(background_texture, 0.0, 0.0, WHITE);

                    
                    draw_texture_ex(ship, ship_x as f32, ship_y as f32, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(12.5, 16.0)), rotation: ship_angle as f32, ..Default::default() });
                    
                    draw_rectangle(10.0, 30.0, screen_size() as f32 - 20.0, screen_size() as f32 - 40.0, Color::from_rgba(0, 0, 0, 100));
                    
                    
                    fonts.draw_text(&format!("SpaceWarp"), (screen_size()/2) as f32 - (fonts.measure_text(&format!("SpaceWarp"), 8).width/2.0), 15.0, 8, WHITE);

                    for (index, option) in options.iter().enumerate() {
                        let mut color = WHITE;
                        if index as i32 == selected_option {
                            color = YELLOW;
                        }
                        fonts.draw_text(&format!("{option}"), (screen_size()/2) as f32 - (fonts.measure_text(&format!("{option}"), 8).width/2.0), (index*20+40) as f32, 8, color);
                    }

                    if animating {
                        draw_rectangle(0.0, 0.0, screen_size() as f32, screen_size() as f32, Color::from_rgba(0, 0, 0, 255-(anim_frames*5) as u8));
                    }

                    if is_key_pressed(KeyCode::Up) {
                        selected_option -= 1;
                        if selected_option < 0 {
                            selected_option = (options.len()-1) as i32;
                        }
                    }
                    if is_key_pressed(KeyCode::Down) {
                        selected_option += 1;
                        selected_option %= options.len() as i32;
                    }

                    if is_key_pressed(KeyCode::Enter) {
                        if options[selected_option as usize] == "Play" {
                            anim_frames = 51;
                            animating = true;
                        } else if options[selected_option as usize] == "Settings" {

                        } 
                        else if options[selected_option as usize] == "Quit" {
                            break;
                        }
                    }
                
                },

                GameStates::Game => {
                    
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

                            level.lock().unwrap().update();

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

                    if is_key_pressed(KeyCode::R) {
                        let mut level = get_level().lock().unwrap();
                        let spawn_point = level.get_spawn_location();
                        let original_state = level.original_state.clone();
                        
                        level.tiles = Vec::new();
                        for tile in original_state.iter() {
                        let owned = tile.clone();
                        level.tiles.push(owned);
                        }

                        player.x = spawn_point.0*8;
                        player.y = spawn_point.1*8;
                        player.jump = 0;
                    }


                    // Debug text

                    if allow_debug {
                        draw_debug_text(&fonts, &target_fps, &player);
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
                },
                GameStates::Win => {

                },
                _ => {}
            }

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

lazy_static! {
    static ref BGM: SoundLoader = SoundLoader::new(&format!("assets/sounds/bgm.wav"));
}
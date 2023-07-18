use macroquad::{prelude::*, audio::{play_sound, PlaySoundParams, stop_sound}};
use macroquad_text::*;

use std::fs::File;
use std::io::prelude::*;

mod canvas;
use canvas::*;

mod player;
use player::Player;

mod utils;
use utils::*;

mod tile;
use tile::Tile;

mod level;
use level::*;

mod menu;
use menu::*;

use lazy_static::lazy_static;

const FONT: &[u8] = include_bytes!("../assets/font.ttf");

#[derive(Clone, Copy)]
pub enum TileState {
    Void,
    Tile,
}

#[derive(Clone, Copy)]
enum SettingsState {
    Pause,
    Menu,
}

#[derive(Clone, Copy)]
enum PlayState {
    Testing,
    Playing,
}

enum GameStates {
    Menu,
    Game(PlayState),
    Pause(PlayState),
    Settings(SettingsState),
    Editor,
}

#[macroquad::main("SpaceWarp: Definitive Edition")]
async fn main() {

    // Keep track of the game state

    let mut game_state: GameStates = GameStates::Menu;
    
    // Create a canvas
    
    let canvas = Canvas2D::new(screen_size() as f32, screen_size() as f32);
    let canvas_editor = Canvas2D::new(screen_size() as f32, screen_size() as f32 + 24_f32);

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

    // let mut player2:Player = Player::new();
    // player2.x = 16;


    // Clock variables
    
    let mut target_fps: u32 = 30;
    let mut accumulator: f32 = 0.0;
    let mut frame: u64 = 0;

    // Load background texture

    let background_texture = load_texture("assets/background.png").await.unwrap();
    let spacewarp_texture = load_texture("assets/spacewarp.png").await.unwrap();

    let mut menu = Menu::new();


    // Debug

    let mut allow_update:bool = false;
    let mut allow_debug:bool = false;

    let mut selected_option: i32 = 0;

    let mut editor_level = [[TileState::Void; 16]; 16];
    let mut spawn_x = 0;
    let mut spawn_y = 0;
    let pause_options = vec!["Back", "Settings", "Menu", "Quit"];
    
    loop {

        let current_canvas = match game_state {
            GameStates::Editor => canvas_editor,
            _ => canvas,
        };

        set_camera(&current_canvas.camera);
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
           
            
            match game_state {

                GameStates::Menu => {

                    while accumulator >= 1.0 / target_fps as f32 {
                        
                        menu.update();
                        
                        accumulator -= 1.0 / target_fps as f32;

                    }

                    match menu.get_action() {
                        MenuEvents::GoToGame => {
                            game_state = GameStates::Game(PlayState::Playing);
                            frame = 0;
                            
                            level.lock().unwrap().unsafe_set_level_from_file("levels/level_1.sw");
                            
                            play_sound(BGM.sound, PlaySoundParams {looped:true, volume: 1.0});
                            
                            player = Player::new();
                        }
                        MenuEvents::GoToEditor => {
                            let (lvl, a, b) = load_editor_saved();
                            editor_level = lvl;
                            spawn_x = a;
                            spawn_y = b;
                            game_state = GameStates::Editor;
                        }
                        MenuEvents::GoToSettings => {}
                        MenuEvents::Quit => {break;}
                        MenuEvents::NoOp => {}
                    }
                    
                    menu.render(&mut fonts);
                
                },

                GameStates::Settings(_) => {

                }

                GameStates::Game(state) => {
                    
                    while accumulator >= 1.0 / target_fps as f32 {
                        
                        // RUN ALL UPDATE FUNCTIONS HERE
                        if allow_update || !allow_debug {
                            
                            allow_update = false;
                            // update the player

                            player.update();
                            //player2.update();

                            level.lock().unwrap().update(frame);

                            frame += 1;
                        
                        }
                        accumulator -= 1.0 / target_fps as f32;
                    }

                    clear_background(BLACK);

                    draw_texture(background_texture, 0.0, 0.0, WHITE);

                    // Draw level

                    level.lock().unwrap().draw();
                    
                    // Draw the player
                    
                    player.draw(frame);
                    //player2.draw(frame);


                    if is_key_pressed(KeyCode::R) {
                        let mut level = get_level().lock().unwrap();
                        let spawn_point = level.get_spawn_location();
                        let original_state = level.original_state.clone();
                        
                        let mut tiles = [[Tile::new('⬜'); 16]; 16];
                        for (y, row) in original_state.iter().enumerate() {
                            for (x, t) in row.iter().enumerate() {
                                let owned = t.clone();
                                tiles[y][x] = owned;
                            }
                        }

                        level.tiles = tiles;
                        

                        player.x = spawn_point.0*8;
                        player.y = spawn_point.1*8;
                        player.jump = 0;
                    }


                    if is_key_pressed(KeyCode::Z) {
                        player.gravity_multiplier *= -1;
                    }

                    // Debug text

                    if allow_debug {
                        draw_debug_text(&fonts, &target_fps, &player);
                    }

                    
                    // Debug features
                    
                    if is_key_pressed(KeyCode::Minus) {
                        target_fps -= 5;
                    }
                    if is_key_pressed(KeyCode::Equal) {
                        target_fps += 5;
                    }
                    if is_key_pressed(KeyCode::Enter) { allow_update = true }
                    
                    if is_key_pressed(KeyCode::F1) {
                        allow_debug = !allow_debug;
                    }
                    if is_key_pressed(KeyCode::F2) {
                        match state {
                            PlayState::Testing => {}
                            PlayState::Playing => {
                                print!("> ");
                                std::io::stdout().flush().expect("Error flushing stdout");
                                let mut line = String::new();
                                std::io::stdin().read_line(&mut line).unwrap();
                                line = String::from(line.trim());
        
                                let command: Vec<&str> = line.split(" ").collect();
        
                                if command[0] == "load" {
                                    level.lock().unwrap().unsafe_set_level_from_file(command[1]);
                                    player = Player::new();
                                }
                            }
                        }
                        
                    }

                    if is_key_down(KeyCode::Escape) {
                        match state {
                            PlayState::Testing => {
                                game_state = GameStates::Editor;
                            }
                            PlayState::Playing => {
                                selected_option = 0;
                                game_state = GameStates::Pause(state);
                            }
                        }
                    } 
                    
                },

                GameStates::Editor => {
                    
                    while accumulator >= 1.0 / target_fps as f32 {accumulator -= 1.0 / target_fps as f32;}

                    clear_background(WHITE);

                    draw_texture(background_texture, 0.0, 0.0, WHITE);
                    
                    // Paint the tiles
                    
                    let (mouse_x, mouse_y) = canvas_editor.mouse_position();
                    
                    let mouse_x = mouse_x as i32;
                    let mouse_y = mouse_y as i32;

                    let gx = mouse_x - mouse_x % 8;
                    let gy = mouse_y - mouse_y % 8;

                    let x: i32 = gx as i32/8;
                    let y: i32 = gy as i32/8;
                    
                    let can_draw = !(x > 15 || y > 15);
                    
                    if can_draw {
                        if is_mouse_button_down(MouseButton::Left) {
                            editor_level[y as usize][x as usize] = TileState::Tile;
                        } else if is_mouse_button_down(MouseButton::Right) {
                            editor_level[y as usize][x as usize] = TileState::Void;
                        } else if is_mouse_button_down(MouseButton::Middle) {
                            spawn_x = x;
                            spawn_y = y;
                        }
                    }

                    // Draw the tiles

                    for (index, row) in editor_level.iter().enumerate() {
                        for (index_2, item) in row.iter().enumerate() {
                            match *item {
                                TileState::Void => {},
                                TileState::Tile => {
                                    draw_texture(IMAGE_WALL_9.texture, index_2 as f32 * 8.0, index as f32 * 8.0, WHITE);
                                },
                            }
                            if index == spawn_y as usize && index_2 == spawn_x as usize {
                                draw_rectangle(index_2 as f32 * 8.0, index as f32 * 8.0, 8.0, 8.0, RED);
                            }
                        }
                    }

                    if can_draw {
                        draw_rectangle(gx as f32, gy as f32, 8.0, 8.0, Color::from_rgba(255, 255, 255, 150));
                    }

                    // Exit

                    if is_key_pressed(KeyCode::Escape) {
                        game_state = GameStates::Menu
                    }

                    // Export

                    if is_key_pressed(KeyCode::Enter) {
                        let mut file = File::create("out.sw").expect("Error writing");
                        let mut string = String::new();
                       
                        for row in editor_level.iter() {
                            for item in row.iter() {
                                match *item {
                                    TileState::Void => {
                                        string.push_str("⬜️");
                                    },
                                    TileState::Tile => {
                                        string.push_str("⏹️");
                                    },
                                }
                            }
                            string.push_str("\n");
                        }


                        string.push_str(format!("-1\n-1\n-1\n-1\n{spawn_x}\n{spawn_y}").as_str());

                        file.write_all(string.as_bytes()).expect("Failed to write file");

                        level.lock().unwrap().unsafe_set_level_from_file("out.sw");

                        player = Player::new();
                        game_state = GameStates::Game(PlayState::Testing);

                    }

                },

                GameStates::Pause(state) => {

                    // Todo: make this its own separate file

                    while accumulator >= 1.0 / target_fps as f32 {accumulator -= 1.0 / target_fps as f32;}

                    draw_texture(background_texture, 0.0, 0.0, WHITE);
                    
                    draw_rectangle(0.0, 0.0, 128.0, 128.0, Color::from_rgba(0,0,0,100));
                    
                    draw_rectangle(10.0, 5.0, screen_size() as f32 - 20.0, screen_size() as f32 - 10.0, Color::from_rgba(0, 0, 0, 100));
                    
                    draw_texture_ex(spacewarp_texture, 23.0, 5.0, WHITE, DrawTextureParams {dest_size: Some(Vec2::new(80.0,35.0)), ..Default::default()});


                    for (index, option) in pause_options.iter().enumerate() {
                        let mut color = WHITE;
                        if index as i32 == selected_option {
                            color = YELLOW;
                        }
                        fonts.draw_text(&format!("{option}"), (screen_size()/2) as f32 - (fonts.measure_text(&format!("{option}"), 8).width/2.0), (index*20+45) as f32, 8, color);
                    }

                    if is_key_pressed(KeyCode::Up) {
                        selected_option -= 1;
                        if selected_option < 0 {
                            selected_option = (pause_options.len()-1) as i32;
                        }
                    }
                    if is_key_pressed(KeyCode::Down) {
                        selected_option += 1;
                        selected_option %= pause_options.len() as i32;
                    }

                    if is_key_pressed(KeyCode::Enter) {
                        if pause_options[selected_option as usize] == "Back" {
                            game_state = GameStates::Game(state);
                        } else if pause_options[selected_option as usize] == "Settings" {

                        } 
                        else if pause_options[selected_option as usize] == "Menu" {
                            menu = Menu::new();
                            game_state = GameStates::Menu;
                            
                            selected_option = 0;
                            stop_sound(BGM.sound);
                        } 
                        else if pause_options[selected_option as usize] == "Quit" {
                            break;
                        }
                    }

                },
            }
        }
        
        set_default_camera();

        match game_state {
            GameStates::Editor => {
                canvas_editor.draw();
            },
            _ => canvas.draw(),
        };
        

        next_frame().await
    }
}

lazy_static! {
    static ref BGM: SoundLoader = SoundLoader::new(&format!("assets/sounds/bgm.wav"));

    static ref IMAGE_FIRE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/fire.png", *PACK));
    static ref IMAGE_WALL_1: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/top.png", *PACK));
    static ref IMAGE_WALL_2: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/bottom.png", *PACK));
    static ref IMAGE_WALL_3: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/left.png", *PACK));
    static ref IMAGE_WALL_4: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/right.png", *PACK));
    static ref IMAGE_WALL_5: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/top-left.png", *PACK));
    static ref IMAGE_WALL_6: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/top-right.png", *PACK));
    static ref IMAGE_WALL_7: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/bottom-left.png", *PACK));
    static ref IMAGE_WALL_8: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/bottom-right.png", *PACK));
    static ref IMAGE_WALL_9: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/square/center.png", *PACK));
    static ref IMAGE_WALL_10: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/bottom/left.png", *PACK));
    static ref IMAGE_WALL_11: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/bottom/center.png", *PACK));
    static ref IMAGE_WALL_12: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/bottom/right.png", *PACK));
    static ref IMAGE_WALL_13: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/top/top.png", *PACK));
    static ref IMAGE_WALL_14: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/top/center.png", *PACK));
    static ref IMAGE_WALL_15: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/top/bottom.png", *PACK));
    static ref IMAGE_WALL_16: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/single.png", *PACK));
    static ref IMAGE_WALL_17: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/top-left.png", *PACK));
    static ref IMAGE_WALL_18: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/top-right.png", *PACK));
    static ref IMAGE_WALL_19: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/bottom-left.png", *PACK));
    static ref IMAGE_WALL_20: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/tiles/corner/bottom-right.png", *PACK));

    static ref BOTTOM_DOOR_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/door/bottom.png", *PACK));
    static ref TOP_DOOR_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/door/top.png", *PACK));

    static ref BOTTOM_DOOR_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/door/bottom.png", *PACK));
    static ref TOP_DOOR_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/door/top.png", *PACK));

    static ref BOTTOM_DOOR_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/door/bottom.png", *PACK));
    static ref TOP_DOOR_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/door/top.png", *PACK));

    static ref KEY_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/key.png", *PACK));
    static ref KEY_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/key.png", *PACK));
    static ref KEY_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/key.png", *PACK));

    static ref BUTTON_RED: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/red/button.png", *PACK));
    static ref BUTTON_BLUE: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/blue/button.png", *PACK));
    static ref BUTTON_YELLOW: ImageLoader = ImageLoader::new(&format!("assets/packs/{}/objects/yellow/button.png", *PACK));
}

pub fn load_editor_saved() -> ([[TileState; 16]; 16], i32, i32) {
    let file = File::open("out.sw");
    return match file {
        Ok(mut f) => {
            let mut l = [[TileState::Void; 16]; 16];
            let mut contents = String::new(); 
            f.read_to_string(&mut contents).unwrap();

                // filter out \u{fe0f} and \u{20e3} (aka: extra emoji characters)

            let filtered_input: String = contents
            .chars()
            .filter(|&c| c != '\u{fe0f}' && c != '\u{20e3}')
            .collect();

            // separate into vec by newline

            let lines: Vec<&str> = filtered_input.lines().collect();

            for (y, line) in lines.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '⏹' {
                        l[y][x] = TileState::Tile;
                    } else if c == '⬜' {
                        l[y][x] = TileState::Void;
                    }
                }
            }

            let x: i32 = lines.iter().skip(20).take(1).next().expect("err reading x").parse().expect("msg");
            let y: i32 = lines.iter().skip(21).take(1).next().expect("err reading y").parse().expect("msg");
                
            

            (l, x, y)
            
        }
        Err(_) => ([[TileState::Void; 16]; 16], 0, 0)
    };    
}
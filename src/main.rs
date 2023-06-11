use macroquad::{prelude::*};

#[macroquad::main("BasicShapes")]
async fn main() {

    let screen_size: f32 = 128.0*3.0;

    request_new_screen_size(screen_size, screen_size);
    
    loop {
        
        // Ensure size is correct

        request_new_screen_size(screen_size, screen_size);
        
        clear_background(LIGHTGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("HELLO", 0.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
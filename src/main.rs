use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let tileset = load_texture("assets/Clubs-88x124.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        
        draw_texture_ex(&tileset,50.0,50.0,WHITE,DrawTextureParams{dest_size:None,source:Some(Rect{x:0.0,y:0.0,w:88.0,h:124.0}),rotation:0.0,flip_x:false,flip_y:false,pivot:None});
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}



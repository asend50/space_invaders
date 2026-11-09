use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::ui::text_button::TextButton;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32, highscore: i32) -> (String, TextureManager, i32, i32 ){
    
    let menu= StillImage::from_preload(
    tm.get_preload("assets/menu.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);
let mut lbl_highscore = Label::new(format!("Highscore: {}", highscore), 350.0, 650.0, 45);
lbl_highscore.with_colors(GREEN, Some(BLACK));

let btn_exit = TextButton::new(
        950.0,
        25.0,
        60.0,
        60.0,
        "X",
        RED,
        ORANGE,
        30
    );

    loop {
        clear_background(WHITE);
        menu.draw();
        lbl_highscore.draw();

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), tm, score, highscore);
        }

        if is_key_pressed(KeyCode::H) {
            return ("help".to_string(), tm, score, highscore);
        }

        if btn_exit.click() {
            return("".to_string(),tm, score, highscore);
        }

        next_frame().await;
    }
}
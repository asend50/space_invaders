use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::ui::text_button::TextButton;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, mut score: i32, mut highscore: i32) -> (String, TextureManager, i32, i32 ){

    let lose= StillImage::from_preload(
    tm.get_preload("assets/lose.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

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

if score > highscore {
    highscore += score;
}
let mut lbl_highscore = Label::new(format!("Highscore: {}", highscore), 350.0, 100.0, 60);
lbl_highscore.with_colors(RED, Some(BLACK));

    loop {
        clear_background(WHITE);
        lose.draw();
        lbl_highscore.draw();


        if is_key_pressed(KeyCode::Space) {
            score = 0;
            return ("menu".to_string(), tm, score, highscore);
        }

        if btn_exit.click() {
            return("".to_string(),tm, score, highscore);
        }

        next_frame().await;
    }
}
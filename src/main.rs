/*
By: <Asen Doiron>
Date: 2026-10-07
Program Details: <The purpose of this program is to allow the player to fight against infinite waves of enemies in a spaceship, gaining points as they get kills, while also preventing the enemies from reaching them.>
*/

mod ui;
mod utils;
mod custom;
mod menu;
mod game;
mod lose;
mod help;

use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "spaceinvaders".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: false,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut tm = TextureManager::new();

    tm.preload_with_loading_screen(&["assets/menu.png", "assets/player.png", "assets/enemy.png", "assets/bullet.png", "assets/enemybullet.png", "assets/gamebackground.png", "assets/gamebars.png", "assets/lose.png", "assets/win.png", "assets/help.png", "assets/heart.png"], None, None).await;

    let mut score = 0;
    let mut highscore = 0;
    
    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen, tm, score, highscore) = match current_screen.as_str() {
                "menu" => menu::run(tm, score, highscore).await,
                "game" => game::run(tm, score, highscore).await,
                "lose" => lose::run(tm, score, highscore).await,
                "help" => help::run(tm, score, highscore).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
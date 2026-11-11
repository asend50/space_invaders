use crate::custom::bullet::Bullet;
use crate::custom::enemy::Enemy;
use crate::custom::health::Health;
use crate::custom::player::Player;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::utils::collision::check_collision;
use crate::utils::preload_image::TextureManager;
use ::rand::Rng;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, mut score: i32, mut highscore: i32) -> (String, TextureManager, i32, i32) {
    let mut player = Player::new(
        500.0,                  // X position
        680.0,                  // Y position
        screen_width() * 0.07,  // Width
        screen_height() * 0.08, // Height
        "assets/player.png",    // Image name
        true,                   // Enable stretching
        1.0,                    // Zoom level (100%)
        vec2(35.0, 15.0),       // Old position
    )
    .await;

    let background = StillImage::from_preload(
        tm.get_preload("assets/gamebackground.png").unwrap(),
        screen_width(),  // Width
        screen_height(), // Height
        0.0,             // X position
        0.0,             // Y position
        true,            // Enable stretching
        1.0,             // Zoom level (100%)
    );

    let gamebars = StillImage::from_preload(
        tm.get_preload("assets/gamebars.png").unwrap(),
        screen_width(),  // Width
        screen_height(), // Height
        0.0,             // X position
        0.0,             // Y position
        true,            // Enable stretching
        1.0,             // Zoom level (100%)
    );

    // Buttons:
    let btn_exit = TextButton::new(950.0, 25.0, 60.0, 60.0, "X", RED, ORANGE, 30);
    let btn_return = TextButton::new(25.0, 25.0, 60.0, 60.0, "<-", BLUE, PURPLE, 30);

    // Labels:
    let mut lbl_score = Label::new(format!("Score:\n{}", score), 25.0, 125.0, 45);
    let mut lbl_highscore = Label::new(format!("Highscore:\n{}", highscore), 815.0, 125.0, 45);
    let mut lbl_wave = Label::new(format!(""), 835.0, 710.0, 45);
    

    // Vectors:
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemybullets: Vec<Bullet> = Vec::new();
    let mut bullets_to_remove: Vec<usize> = Vec::new();
    let mut enemy_bullets_to_remove: Vec<usize> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut hearts: Vec<Health> = Vec::new();

    // Variables:
    let mut shoot_cooldown = 0.0;
    let mut enemyshoot_cooldown = 2.0;
    let mut columns;
    let mut rows;
    let mut enemy_speed = 0.2;let mut wave = 1;
    let spacing_x = screen_width() * 0.08;
    let spacing_y = screen_height() * 0.09;
    let min_x = 230.0;
    let max_x = 740.0;
    let mut rng = ::rand::rng();
    let random_columns: i32 = rng.random_range(5..=6);
    let random_rows: i32 = rng.random_range(4..=6);
    let mut player_hp = 3;

    for i in 0..player_hp {
        let heart = Health::from_preload(
            tm.get_preload("assets/heart.png").unwrap(),
            35.0 + i as f32 * 55.0,
            675.0,
            45.0,
            45.0,
            true,
            1.0,
        );

        hearts.push(heart);
    }

    // Enemy Creation:
    columns = random_columns;
    rows = random_rows;

    for row in 0..rows {
        for col in 0..columns {
            let enemy = Enemy::from_preload(
                tm.get_preload("assets/enemy.png").unwrap(),
                240.0 + col as f32 * spacing_x,
                25.0 + row as f32 * spacing_y,
                screen_width() * 0.055,
                screen_height() * 0.065,
                true,
                1.0,
            );
            enemies.push(enemy);
        }
    }

    loop {
        // Drawing
        clear_background(WHITE);
        background.draw();
        player.get_image().draw();
        for enemy in &enemies {
            enemy.get_image().draw();
        }

        player.keypress();
        player.move_player();
        lbl_wave.set_text(format!("Wave: {}", wave));

        // Player Collision
        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &gamebars, 1) {
                player.move_back_x();
            }
        }

        // Player Bullet Code
        let playershottime = get_frame_time();
        shoot_cooldown -= playershottime;

        if is_key_down(KeyCode::Space) && shoot_cooldown <= 0.0 {
            shoot_cooldown = 0.75;

            let bullet = Bullet::from_preload(
                tm.get_preload("assets/bullet.png").unwrap(),
                player.image.get_x() + 25.0,
                player.image.get_y(),
                screen_width() * 0.02,
                screen_height() * 0.05,
                true,
                1.0,
                3.0,
            );

            bullets.push(bullet);
        }

        for bullet_index in 0..bullets.len() {
            let bullet = &mut bullets[bullet_index];
            bullet.playershoot();
            bullet.get_image().draw();

            if bullet.image.get_y() < -100.0 {
                bullet.image.clear();
                bullet.move_dir = vec2(0.0, 0.0);
                bullets_to_remove.push(bullet_index);
            }

            // Enemy and Bullet collsion to kill enemies.
            for enemy_index in 0..enemies.len() {
                if check_collision(bullet.get_image(), enemies[enemy_index].get_image(), 1) {
                    enemies.remove(enemy_index);
                    bullets_to_remove.push(bullet_index);
                    score += 100;
                    lbl_score.set_text(format!("Score:\n{}", score));
                    if score > highscore {
                        highscore = score;
                    }
                    lbl_highscore.set_text(format!("Highscore:\n{}", highscore));
                    break;
                }
            }
        }

        // Enemy Bullet Code
        enemyshoot_cooldown -= playershottime;

        if !enemies.is_empty() && enemyshoot_cooldown <= 0.0 {
            enemyshoot_cooldown = 1.5;

            let enemy_index = rng.random_range(0..enemies.len());
            let randomenemy = &enemies[enemy_index];

            let enemybullet = Bullet::from_preload(
                tm.get_preload("assets/enemybullet.png").unwrap(),
                randomenemy.image.get_x() + 25.0,
                randomenemy.image.get_y(),
                screen_width() * 0.02,
                screen_height() * 0.05,
                true,
                1.0,
                3.0,
            );

            enemybullets.push(enemybullet);
        }

        for enemybullet_index in 0..enemybullets.len() {
            let enemybullet = &mut enemybullets[enemybullet_index];
            enemybullet.enemyshoot();
            enemybullet.get_image().draw();

            if enemybullet.image.get_y() > screen_height() + 100.0 {
                enemybullet.image.clear();
                enemybullet.move_dir = vec2(0.0, 0.0);
                enemy_bullets_to_remove.push(enemybullet_index);
                continue;
            }

            if check_collision(enemybullet.get_image(), player.get_image(), 1) {
                enemy_bullets_to_remove.push(enemybullet_index);

                if !hearts.is_empty() {
                    hearts.pop();
                }

                player_hp -= 1;

                if player_hp <= 0 {
                    return ("lose".to_string(), tm, score, highscore);
                }

                break;
            }
        }

        for &bullet_index in &bullets_to_remove {
            bullets.remove(bullet_index);
        }
        bullets_to_remove.clear();

        for &enemybullet_index in &enemy_bullets_to_remove {
            enemybullets.remove(enemybullet_index);
        }
        enemy_bullets_to_remove.clear();

        // Next Wave code
        if enemies.is_empty() {
            wave += 1;
            let random_columns: i32 = rng.random_range(5..=6);
            let random_rows: i32 = rng.random_range(3..=4);

            columns = random_columns;
            rows = random_rows;
            for row in 0..rows {
                for col in 0..columns {
                    let enemy = Enemy::from_preload(
                        tm.get_preload("assets/enemy.png").unwrap(),
                        240.0 + col as f32 * spacing_x,
                        25.0 + row as f32 * spacing_y,
                        screen_width() * 0.055,
                        screen_height() * 0.065,
                        true,
                        1.0,
                    );
                    enemies.push(enemy);
                }
            }

            if player_hp < 3 {
                player_hp += 1;
                let heart = Health::from_preload(
                    tm.get_preload("assets/heart.png").unwrap(),
                    35.0 + hearts.len() as f32 * 55.0,
                    675.0,
                    45.0,
                    45.0,
                    true,
                    1.0,
                );
                hearts.push(heart);
            }
        }

        // Enemy Movement
        for enemy in &mut enemies {
            enemy.image.set_x(enemy.image.get_x() + enemy_speed * 2.0);
        }


        let mut left = 0.0;
        let mut right = 0.0;

        for enemy in &enemies {
            let x = enemy.image.get_x();

            if x < left || left == 0.0 {
                left = x;
            }
            if x > right {
                right = x;
            }
        }

        if left <= min_x || right >= max_x {
            enemy_speed *= -1.0;
            for enemy in &mut enemies {
                enemy.image.set_y(enemy.image.get_y() + 10.0);
            }
        }

        // Lose Code
        if enemies.iter().any(|enemy| enemy.image.get_y() >= 650.0) {
            return ("lose".to_string(), tm, score, highscore);
        }

        // Drawing Visuals, Scores, and Buttons.
        gamebars.draw();
        lbl_score.draw();
        lbl_highscore.draw();
        lbl_wave.draw();

        for heart in &hearts {
            heart.get_image().draw();
        }

        if btn_exit.click() {
            return ("".to_string(), tm, score, highscore);
        }

        if btn_return.click() {
            score = 0;
            return ("menu".to_string(), tm, score, highscore);
        }

        next_frame().await;
    }
}

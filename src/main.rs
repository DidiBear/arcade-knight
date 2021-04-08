//! The Arcade Knight game

#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
#![allow(clippy::future_not_send)]

use std::time::{SystemTime, UNIX_EPOCH};

use enemy::Enemy;
use macroquad::{prelude::*, rand::srand};
use player::Player;
use screen_drawer::{load_scalable_texture, ScreenDrawer};

mod character;
mod enemy;
mod player;
mod screen_drawer;

/// Width of the game.
pub const GAME_WIDTH: f32 = 250.;
/// Height of the game.
pub const GAME_HEIGHT: f32 = 250.;
/// Movement speed of enemies.
pub const ENEMY_SPEED: f32 = 15.;

#[macroquad::main("Arcade Knight")]
async fn main() {
    seed_random();

    let player_texture = load_scalable_texture("resources/player.png").await;
    let enemy_texture = load_scalable_texture("resources/enemy.png").await;

    let player = Player::new(player_texture);
    let mut enemy = Enemy::new_random(enemy_texture);
    let screen_drawer = ScreenDrawer::new(vec2(GAME_WIDTH, GAME_HEIGHT));

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        enemy.update();

        screen_drawer.draw_scaled(|| {
            clear_background(LIME);
            player.draw();
            enemy.draw();
        });

        next_frame().await;
    }
}

/// Seed the random generator with the current time.
fn seed_random() {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    srand(current_time);
}

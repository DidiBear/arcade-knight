//! The Arcade Knight game

#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
#![allow(clippy::future_not_send, clippy::cast_precision_loss)]

use std::time::{SystemTime, UNIX_EPOCH};

use enemy::{Enemy, Spawner};
use life_bar::LifeBar;
use macroquad::{prelude::*, rand::srand};
use player::{Player, Slash};
use screen_drawer::{load_scalable_texture, ScreenDrawer};

mod character;
mod enemy;
mod life_bar;
mod player;
mod screen_drawer;

/// Width of the game.
pub const GAME_WIDTH: f32 = 250.;
/// Height of the game.
pub const GAME_HEIGHT: f32 = 250.;
/// Movement speed of enemies.
pub const ENEMY_SPEED: f32 = 40.;
/// Initial delay between each enemy spawn.
pub const INITIAL_SPAWN_DELAY: f64 = 1.;
/// Initial amount of life the player has.
pub const LIVES: u32 = 3;

#[macroquad::main("Arcade Knight")]
async fn main() {
    seed_random();

    let player_texture = load_scalable_texture("resources/player.png").await;
    let enemy_texture = load_scalable_texture("resources/enemy.png").await;
    let heart_texture = load_scalable_texture("resources/heart.png").await;
    let empty_heart_texture = load_scalable_texture("resources/empty_heart.png").await;

    let mut life_bar = LifeBar::new(LIVES, heart_texture, empty_heart_texture);

    let mut player = Player::new(player_texture);
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut spawner = Spawner::new(INITIAL_SPAWN_DELAY);

    let screen_drawer = ScreenDrawer::new(vec2(GAME_WIDTH, GAME_HEIGHT));

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        player.update_direction();

        let attack = if is_key_down(KeyCode::Space) {
            Some(player.slash_attack())
        } else {
            None
        };

        spawner.tick_and_spawn(|| enemies.push(Enemy::new_random(enemy_texture)));
        enemies.iter_mut().for_each(Enemy::update);

        enemies.retain(|enemy| {
            if attack.as_ref().map_or(false, |attack| attack.kill(enemy)) {
                return false;
            }
            if enemy.character.collide(&player.character) {
                life_bar.decrement();
                return false;
            }
            true
        });

        screen_drawer.draw_scaled(|| {
            clear_background(LIME);
            player.character.draw();
            enemies.iter().for_each(|enemy| enemy.character.draw());
            life_bar.draw();
            attack.as_ref().map(Slash::draw);
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

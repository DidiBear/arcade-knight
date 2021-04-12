//! The Arcade Knight game

#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
#![allow(
    clippy::future_not_send,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::eval_order_dependence
)]

use std::time::{SystemTime, UNIX_EPOCH};

use enemy::Enemy;
use life_bar::LifeBar;
use macroquad::{prelude::*, rand::srand};
use player::{Player, Slash};
use resources::{Fonts, Textures};
use screen_drawer::ScreenDrawer;
use timers::{Cooldown, Timer};

mod character;
mod enemy;
mod life_bar;
mod player;
mod resources;
mod screen_drawer;
mod timers;

/// Width of the game.
pub const GAME_WIDTH: f32 = 250.;
/// Height of the game.
pub const GAME_HEIGHT: f32 = 250.;
/// Margin for the score text.
pub const MARGIN: f32 = 4.;
/// Movement speed of enemies.
pub const ENEMY_SPEED: f32 = 40.;
/// Initial delay between each enemy spawn.
pub const INITIAL_SPAWN_DELAY: f64 = 1.;
/// Duration of the cooldown between attacks.
pub const SLASH_COOLDOWN: f64 = 0.5;
/// Initial amount of life the player has.
pub const LIVES: u32 = 3;

#[macroquad::main(window_conf)]
async fn main() {
    seed_random();
    let mut game = Game::load().await;

    game.launch().await;
}

struct Game {
    textures: Textures,
    fonts: Fonts,
    screen_drawer: ScreenDrawer,
    max_score: u32,
}

impl Game {
    async fn load() -> Self {
        Self {
            textures: Textures::load().await,
            fonts: Fonts::load().await,
            screen_drawer: ScreenDrawer::new(vec2(GAME_WIDTH, GAME_HEIGHT)),
            max_score: 0,
        }
    }

    async fn launch(&mut self) {
        loop {
            self.menu().await;
            let score = self.game().await;

            self.max_score = self.max_score.max(score);
        }
    }

    async fn menu(&self) {
        loop {
            if is_key_pressed(KeyCode::Space) {
                return;
            }

            self.screen_drawer.draw_scaled(|| {
                draw_texture(self.textures.background, 0., 0., WHITE);

                let x = GAME_WIDTH / 2.;
                let y = GAME_HEIGHT / 2.;

                let title = "Arcade knight";
                let score = &format!("Max score: {}", self.max_score);
                let start = "Press space to start";

                Fonts::draw_centered(title, x, y, self.fonts.sized(20));
                Fonts::draw_centered(score, x, y + 15., self.fonts.sized(8));
                Fonts::draw_centered(start, x, y + 50., self.fonts.sized(8));
            });

            next_frame().await;
        }
    }

    async fn game(&self) -> u32 {
        let mut score: u32 = 0;
        let mut life_bar = LifeBar::new(LIVES, self.textures.heart, self.textures.empty_heart);

        let mut player = Player::new(self.textures.player);
        let mut enemies: Vec<Enemy> = Vec::new();

        let mut slash_cooldown = Cooldown::from_seconds(SLASH_COOLDOWN);
        let mut spawner = Timer::from_seconds(INITIAL_SPAWN_DELAY);

        loop {
            player.update_direction();

            let attack = if is_key_pressed(KeyCode::Space) && slash_cooldown.available() {
                slash_cooldown.start();
                Some(player.slash_attack())
            } else {
                None
            };

            if spawner.tick_and_finished() {
                enemies.push(Enemy::new_random(self.textures.enemy));
            }
            enemies.iter_mut().for_each(Enemy::update);

            enemies.retain(|enemy| {
                if attack.as_ref().map_or(false, |attack| attack.kill(enemy)) {
                    score += 10;
                    slash_cooldown.reset();
                    spawner.delay = 1.0 / get_time().mul_add(0.1, 0.5);
                    return false;
                }
                if enemy.character.collide(&player.character) {
                    life_bar.decrement();
                    return false;
                }
                true
            });

            self.screen_drawer.draw_scaled(|| {
                clear_background(LIME);
                draw_texture(self.textures.background, 0., 0., WHITE);
                player.character.draw();
                enemies.iter().for_each(|enemy| enemy.character.draw());
                life_bar.draw();
                attack.as_ref().map(Slash::draw);

                let score = &format!("Score: {}", score);
                Fonts::draw_left(score, GAME_WIDTH - MARGIN, MARGIN, self.fonts.sized(8));
            });

            if life_bar.is_empty() {
                return score;
            }

            next_frame().await;
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Arcade Knight".to_owned(),
        window_width: (GAME_WIDTH * 3.) as i32,
        window_height: (GAME_HEIGHT * 3.) as i32,
        ..Conf::default()
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

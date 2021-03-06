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

use enemy::Enemy;
use life_bar::LifeBar;
use macroquad::{prelude::*, rand::srand};
use player::Player;
use resources::{Animations, Fonts, Textures};
use screen_drawer::ScreenDrawer;
use timers::{Cooldown, Timer};

mod animation;
mod character;
mod direction;
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
pub const ATTACK_COOLDOWN: f64 = 0.3;
/// Initial amount of life the player has.
pub const LIVES: u32 = 5;

#[macroquad::main(window_conf)]
async fn main() {
    Game::load().await.start().await;
}

struct Game {
    textures: Textures,
    fonts: Fonts,
    animations: Animations,
    screen_drawer: ScreenDrawer,
    max_score: u32,
}

impl Game {
    async fn load() -> Self {
        let textures = Textures::load().await;
        Self {
            fonts: Fonts::load().await,
            animations: Animations::new(&textures),
            textures,
            screen_drawer: ScreenDrawer::new(GAME_WIDTH, GAME_HEIGHT),
            max_score: 0,
        }
    }

    async fn start(&mut self) {
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
        srand(get_time().to_bits());

        let mut score: u32 = 0;
        let mut life_bar = LifeBar::new(LIVES, &self.textures);

        let mut player = Player::new(12., 12., &self.textures);
        let mut enemies: Vec<Enemy> = Vec::new();

        let mut attack_cooldown = Cooldown::from_seconds(ATTACK_COOLDOWN);
        let mut enemy_spawner = Timer::from_seconds(INITIAL_SPAWN_DELAY);

        loop {
            player.update_direction();
            player.animate_attack();

            if is_key_pressed(KeyCode::Space) && attack_cooldown.available() {
                attack_cooldown.start();
                player.start_attack(&self.animations);
            }
            if enemy_spawner.tick_and_finished() {
                enemies.push(Enemy::new_random(16., 16., &self.animations));
            }

            for enemy in &mut enemies {
                enemy.move_and_animate();

                if player.kill(enemy) {
                    score += 10;
                    attack_cooldown.reset();
                    enemy_spawner.delay = 1.0 / get_time().mul_add(0.1, 0.5);
                    enemy.alive = false;
                }
                if enemy.character.collide(&player.character) {
                    life_bar.decrement();
                    enemy.alive = false;
                }
            }

            enemies.retain(|enemy| enemy.alive);

            self.screen_drawer.draw_scaled(|| {
                clear_background(LIME);
                draw_texture(self.textures.background, 0., 0., WHITE);
                let Rect { x, y, w, h } = player.slash_attack();
                draw_rectangle(x, y, w, h, SKYBLUE);
                player.draw();
                enemies.iter().for_each(Enemy::draw);
                life_bar.draw();

                if cfg!(debug_assertions) {

                    player.character.draw_hit_box();
                    enemies.iter().for_each(|e| e.character.draw_hit_box());
                }

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

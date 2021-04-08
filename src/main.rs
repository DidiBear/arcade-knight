//! The Arcade Knight game

#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
#![allow(clippy::future_not_send)]

use macroquad::prelude::*;
use player::Player;

mod player;

/// Width of the game.
pub const GAME_WIDTH: f32 = 250.;
/// Height of the game.
pub const GAME_HEIGHT: f32 = 250.;

#[macroquad::main("Arcade Knight")]
async fn main() {
    let player_texture = load_scalable_texture("resources/player.png").await;

    let player = Player::new(player_texture);
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(LIME);
        player.draw();

        next_frame().await;
    }
}

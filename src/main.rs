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

#[macroquad::main("Arcade Knight")]
async fn main() {
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

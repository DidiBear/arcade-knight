use macroquad::prelude::*;

use crate::{character::Character, GAME_HEIGHT, GAME_WIDTH};

pub struct Player {
    pub character: Character,
}

impl Player {
    /// Creates the player entity centered in the middle of the screen.
    pub fn new(texture: Texture2D) -> Self {
        Self {
            character: Character::new(GAME_WIDTH / 2., GAME_HEIGHT / 2., vec2(1.0, 0.0), texture),
        }
    }

}

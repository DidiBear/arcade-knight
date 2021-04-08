use macroquad::prelude::*;

use crate::{GAME_HEIGHT, GAME_WIDTH, character::Character, enemy::Enemy};

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

    /// Returns a slash attack positioned at the player's direction   
    pub fn slash_attack(&self) -> Slash {
        let Character {
            body, direction, ..
        } = self.character;

        Slash(body.offset(direction * body.size()))
    }
}

/// A slash attack.
pub struct Slash(Rect);

impl Slash {
    /// Returns true if the slash attack kill the given character.
    pub fn kill(&self, enemy: &Enemy) -> bool {
        self.0.overlaps(&enemy.character.body)
    }

    pub fn draw(&self) {
        let Rect { x, y, w, h } = self.0;
        draw_rectangle(x, y, w, h, BLUE);
    }
}

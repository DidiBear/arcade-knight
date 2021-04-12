use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{character::Character, ENEMY_SPEED, GAME_HEIGHT, GAME_WIDTH};

pub struct Enemy {
    pub character: Character,
}

impl Enemy {
    /// Creates a random enemy in one side of the screen.
    pub fn new_random(texture: Texture2D) -> Self {
        let default_values = vec![
            (vec2(0.0, 1.0), GAME_WIDTH / 2., 0.),            // Top
            (vec2(0.0, -1.0), GAME_WIDTH / 2., GAME_HEIGHT),  // Bottom
            (vec2(-1.0, 0.0), GAME_WIDTH, GAME_HEIGHT / 2.0), // Right
            (vec2(1.0, 0.0), 0., GAME_HEIGHT / 2.),           // Left
        ];

        let (direction, x, y) = default_values.choose().unwrap();

        Self {
            character: Character::new(*x, *y, *direction, texture),
        }
    }

    /// Moves the enemy following its direction.
    pub fn update(&mut self) {
        self.character.move_body(ENEMY_SPEED);
    }
}

use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{
    character::Character,
    direction::{DOWN, LEFT, RIGHT, UP},
    ENEMY_SPEED, GAME_HEIGHT, GAME_WIDTH,
};

pub struct Enemy {
    pub character: Character,
}

impl Enemy {
    /// Creates a random enemy in one side of the screen.
    pub fn new_random(texture: Texture2D) -> Self {
        let default_values = vec![
            (DOWN, GAME_WIDTH / 2., 0.),           // Top
            (UP, GAME_WIDTH / 2., GAME_HEIGHT),    // Bottom
            (LEFT, GAME_WIDTH, GAME_HEIGHT / 2.0), // Right
            (RIGHT, 0., GAME_HEIGHT / 2.),         // Left
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

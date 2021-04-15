use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{
    animation::Animation, character::Character, direction::Direction, resources::Animations,
    ENEMY_SPEED, GAME_HEIGHT, GAME_WIDTH,
};

pub struct Enemy {
    pub character: Character,
    pub animation: Animation,
    pub alive: bool,
}

impl Enemy {
    /// Creates a random enemy placed in one side of the screen.
    pub fn new_random(w: f32, h: f32, animations: &Animations) -> Self {
        let directions: Vec<_> = Direction::iter().collect();
        let direction = *directions.choose().unwrap();

        let (x, y) = match direction {
            Direction::Down => (GAME_WIDTH / 2., 0.),           // Top
            Direction::Up => (GAME_WIDTH / 2., GAME_HEIGHT),    // Bottom
            Direction::Left => (GAME_WIDTH, GAME_HEIGHT / 2.0), // Right
            Direction::Right => (0., GAME_HEIGHT / 2.),         // Left
        };

        Self {
            character: Character::new(x, y, w, h, direction),
            animation: animations.enemy_walking(direction),
            alive: true,
        }
    }

    /// Moves the enemy following its direction and update the animation.
    pub fn move_and_animate(&mut self) {
        self.character.move_body(ENEMY_SPEED);
        self.animation.tick();
    }

    pub fn draw(&self) {
        let (x, y) = self.character.position();
        self.animation.draw_current_centered(x, y)
    }
}

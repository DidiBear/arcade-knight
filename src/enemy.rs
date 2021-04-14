use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{
    animation::Animation,
    character::Character,
    direction::{Side, DOWN, LEFT, RIGHT, UP},
    resources::Animations,
    ENEMY_SPEED, GAME_HEIGHT, GAME_WIDTH,
};

pub struct Enemy {
    pub character: Character,
    pub animation: Animation,
    pub alive: bool,
}

impl Enemy {
    /// Creates a random enemy in one side of the screen.
    pub fn new_random(w: f32, h: f32, animations: &Animations) -> Self {
        let sides = vec![Side::Up, Side::Down, Side::Left, Side::Right];
        let direction_side = *sides.choose().unwrap();

        let (direction, x, y) = match direction_side {
            Side::Down => (DOWN, GAME_WIDTH / 2., 0.),           // Top
            Side::Up => (UP, GAME_WIDTH / 2., GAME_HEIGHT),      // Bottom
            Side::Left => (LEFT, GAME_WIDTH, GAME_HEIGHT / 2.0), // Right
            Side::Right => (RIGHT, 0., GAME_HEIGHT / 2.),        // Left
        };

        Self {
            character: Character::new(x, y, w, h, direction),
            animation: animations.enemy_walking(direction_side),
            alive: true,
        }
    }

    /// Moves the enemy following its direction and update the animation.
    pub fn update(&mut self) {
        self.character.move_body(ENEMY_SPEED);
        self.animation.tick();
    }

    pub fn draw(&self) {
        let (x, y) = self.character.position();
        self.animation.draw_current_centered(x, y)
    }
}

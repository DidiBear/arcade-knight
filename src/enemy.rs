use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{
    animation::Animation,
    character::Character,
    direction::{DOWN, LEFT, RIGHT, UP},
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
        let default_values = vec![
            (DOWN, GAME_WIDTH / 2., 0., &animations.enemy_bottom), // Top
            (UP, GAME_WIDTH / 2., GAME_HEIGHT, &animations.enemy_up), // Bottom
            (LEFT, GAME_WIDTH, GAME_HEIGHT / 2.0, &animations.enemy_left), // Right
            (RIGHT, 0., GAME_HEIGHT / 2., &animations.enemy_right), // Left
        ];

        let (direction, x, y, animation) = *default_values.choose().unwrap();

        Self {
            character: Character::new(x, y, w, h, direction),
            animation: animation.clone(),
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

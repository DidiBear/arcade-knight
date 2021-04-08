use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{ENEMY_SPEED, GAME_HEIGHT, GAME_WIDTH};

pub struct Enemy {
    texture: Texture2D,
    body: Rect,
    direction: Vec2,
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
        let (width, height) = (texture.width(), texture.height());

        // Center the position to the middle of the texture
        let (x, y) = (x - width / 2., y - height / 2.);

        Self {
            texture,
            body: Rect::new(x, y, width, height),
            direction: *direction,
        }
    }

    /// Moves the enemy following its direction.
    pub fn update(&mut self, delta_time: f32) {
        self.body = self.body.offset(self.direction * delta_time * ENEMY_SPEED);
    }

    /// Draws the enemy texture.
    pub fn draw(&self) {
        let Rect { x, y, w, h } = self.body;

        draw_texture(self.texture, x, y, WHITE);

        #[cfg(debug_assertions)]
        draw_rectangle_lines(x, y, w, h, 2., RED);
    }
}

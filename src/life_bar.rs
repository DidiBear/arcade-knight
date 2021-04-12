use macroquad::prelude::*;

pub struct LifeBar {
    max_lives: u32,
    lives: u32,
    full_texture: Texture2D,
    empty_texture: Texture2D,
}

impl LifeBar {
    pub const fn new(max_lives: u32, full_texture: Texture2D, empty_texture: Texture2D) -> Self {
        Self {
            max_lives,
            lives: max_lives,
            full_texture,
            empty_texture,
        }
    }

    /// Draws full hearts for remaining lives and empty hearts for lost ones.
    pub fn draw(&self) {
        for index in 0..self.max_lives {
            let texture = if index < self.lives {
                self.full_texture
            } else {
                self.empty_texture
            };

            let margin: f32 = 2.0;
            let x = (index as f32).mul_add(texture.width(), margin);
            draw_texture(texture, x, 0., WHITE);
        }
    }

    pub fn decrement(&mut self) {
        self.lives = self.lives.saturating_sub(1);
    }

    pub const fn is_empty(&self) -> bool {
        self.lives == 0
    }
}

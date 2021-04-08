use macroquad::prelude::*;

use crate::{GAME_HEIGHT, GAME_WIDTH};

pub struct Player {
    texture: Texture2D,
    body: Rect,
}

impl Player {
    /// Creates the player entity centered in the middle of the screen.
    pub fn new(texture: Texture2D) -> Self {
        let (w, h) = (texture.width(), texture.height());

        // Center the player in the middle of the screen
        let x = GAME_WIDTH / 2. - w / 2.;
        let y = GAME_HEIGHT / 2. - h / 2.;

        Self {
            texture,
            body: Rect::new(x, y, w, h),
        }
    }

    /// Draws the player texture.
    pub fn draw(&self) {
        let Rect { x, y, w, h } = self.body;

        draw_texture(self.texture, x, y, WHITE);

        #[cfg(debug_assertions)]
        draw_rectangle_lines(x, y, w, h, 2., RED);
    }
}

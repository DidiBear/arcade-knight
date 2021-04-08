use macroquad::prelude::*;

pub struct Character {
    texture: Texture2D,
    body: Rect,
    direction: Vec2,
}

impl Character {
    /// Creates a character with a body centered around its position.
    pub fn new(x: f32, y: f32, direction: Vec2, texture: Texture2D) -> Self {
        let (w, h) = (texture.width(), texture.height());

        // Center the position in the middle of the screen
        let x = x - w / 2.;
        let y = y - h / 2.;

        Self {
            texture,
            body: Rect::new(x, y, w, h),
            direction,
        }
    }

    /// Draws the character and its hit-box in debug mode.
    pub fn draw(&self) {
        let Rect { x, y, w, h } = self.body;

        draw_texture(self.texture, x, y, WHITE);

        #[cfg(debug_assertions)]
        draw_rectangle_lines(x, y, w, h, 2., RED);
    }

    /// Moves the body following the direction.
    pub fn move_body(&mut self, speed: f32) {
        self.body = self.body.offset(self.direction * get_frame_time() * speed);
    }
}
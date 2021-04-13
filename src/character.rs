use macroquad::prelude::*;

use crate::direction::Direction;

pub struct Character {
    pub body: Rect,
    pub direction: Direction,
}

impl Character {
    /// Creates a character with the hit box of the given size (w, h) centered
    /// in the given position.   
    pub fn new(x: f32, y: f32, w: f32, h: f32, direction: Direction) -> Self {
        Self {
            body: Rect::new(x - w / 2., y - h / 2., w, h),
            direction,
        }
    }

    /// Draws the hit-box of the character.
    pub fn draw_hit_box(&self) {
        let Rect { x, y, w, h } = self.body;
        draw_rectangle_lines(x, y, w, h, 2., RED);
    }

    /// Returns the position of the character in the middle of the hit-box.
    pub fn position(&self) -> (f32, f32) {
        let Rect { x, y, w, h } = self.body;

        (x + w / 2., y + h / 2.)
    }

    /// Moves the body following the direction.
    pub fn move_body(&mut self, speed: f32) {
        let translation = self.direction.vector * get_frame_time() * speed;
        self.body = self.body.offset(translation);
    }

    /// Returns true if both characters are in collision.
    pub fn collide(&self, other: &Self) -> bool {
        self.body.overlaps(&other.body)
    }
}

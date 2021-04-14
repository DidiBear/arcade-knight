use std::rc::Rc;

use macroquad::prelude::*;

use crate::timers::Timer;

pub struct TextureAtlas {
    texture: Texture2D,
    tiles: Vec<Rect>,
}

impl TextureAtlas {
    /// Creates atlas with all the tiles indexes following the order in the grid.
    pub fn from_grid(
        texture: Texture2D,
        tile_size: (f32, f32),
        columns: usize,
        rows: usize,
    ) -> Self {
        let (w, h) = tile_size;

        assert!((columns as f32) * w <= texture.width(), "Too many columns");
        assert!((rows as f32) * h <= texture.height(), "Too many rows");

        let mut tiles = Vec::new();
        for y in 0..rows {
            for x in 0..columns {
                let (x, y) = (x as f32, y as f32);
                tiles.push(Rect::new(x * w, y * h, w, h));
            }
        }

        Self { texture, tiles }
    }

    /// Draws the tile at the given index centered at the given position.
    pub fn draw_tile_centered(&self, tile_index: usize, x: f32, y: f32) {
        let tile = self.tiles.get(tile_index).expect("Tile not found");
        let (x, y) = (x - tile.w / 2., y - tile.h / 2.);

        draw_texture_ex(self.texture, x, y, WHITE, draw_tile_params(tile));
    }
}

fn draw_tile_params(tile: &Rect) -> DrawTextureParams {
    DrawTextureParams {
        source: Some(*tile),
        ..DrawTextureParams::default()
    }
}

#[derive(Clone)]
pub struct Animation {
    texture_atlas: Rc<TextureAtlas>,
    /// Indexes of the tiles in the atlas composing the animation.  
    tile_indexes: Vec<usize>,
    /// Index of the current frame.
    pub current_frame: usize,
    /// Timer keeping track of the frame durations.
    frame_timer: Timer,
    /// Whether or not the animation is repeating infinitely.
    repeating: bool,
}

impl Animation {
    pub fn new(
        texture_atlas: Rc<TextureAtlas>,
        tile_indexes: Vec<usize>,
        frame_seconds: f64,
        repeating: bool,
    ) -> Self {
        Self {
            texture_atlas,
            tile_indexes,
            repeating,
            frame_timer: Timer::from_seconds(frame_seconds),
            current_frame: 0,
        }
    }

    /// Draw the current frame of the animation centered around the given position.
    pub fn draw_current_centered(&self, x: f32, y: f32) {
        assert!(!self.is_finished(), "Cannot draw a finished animation");
        let tile_index = self.tile_indexes[self.current_frame]; // index checked above

        self.texture_atlas.draw_tile_centered(tile_index, x, y);
    }

    /// Updates the current frame.
    pub fn tick(&mut self) -> &Self {
        if self.frame_timer.tick_and_finished() {
            self.current_frame += 1;

            if self.repeating && self.is_finished() {
                self.restart();
            }
        }
        self
    }

    /// Whether or not the animation is finished.
    pub fn is_finished(&self) -> bool {
        self.current_frame >= self.tile_indexes.len()
    }

    /// Restart the animation to the initial frame.
    pub fn restart(&mut self) {
        self.current_frame = 0;
        self.frame_timer.restart();
    }
}

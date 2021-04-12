use macroquad::prelude::*;

use crate::screen_drawer::load_scalable_texture;

pub struct Textures {
    pub player: Texture2D,
    pub enemy: Texture2D,
    pub heart: Texture2D,
    pub empty_heart: Texture2D,
    pub background: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        Self {
            player: load_scalable_texture("resources/player.png").await,
            enemy: load_scalable_texture("resources/enemy.png").await,
            heart: load_scalable_texture("resources/heart.png").await,
            empty_heart: load_scalable_texture("resources/empty_heart.png").await,
            background: load_scalable_texture("resources/background.png").await,
        }
    }
}

pub struct Fonts {
    pub font: Font,
}

impl Fonts {
    pub async fn load() -> Self {
        Self {
            font: load_ttf_font("resources/Kenney Pixel Square.ttf").await,
        }
    }

    pub const fn sized(&self, font_size: u16) -> TextParams {
        TextParams {
            font: self.font,
            font_size,
            font_scale: 1.,
            color: WHITE,
        }
    }

    pub fn draw_left(text: &str, x: f32, y: f32, params: TextParams) {
        let size = measure(text, params);

        let x = x - size.width;
        let y = y + size.height;

        draw_text_ex(text, x, y, params);
    }
}

fn measure(text: &str, params: TextParams) -> TextDimensions {
    measure_text(text, Some(params.font), params.font_size, params.font_scale)
}

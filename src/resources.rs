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

pub async fn load_text_params() -> TextParams {
    TextParams {
        font: load_ttf_font("resources/Kenney Pixel Square.ttf").await,
        font_size: 8,
        font_scale: 1.,
        color: WHITE,
    }
}

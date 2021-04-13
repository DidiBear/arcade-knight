use macroquad::prelude::*;

use crate::{
    animation::{Animation, TextureAtlas},
    screen_drawer::load_scalable_texture,
};

pub struct Textures {
    pub player_atlas: TextureAtlas,
    pub enemy_atlas: TextureAtlas,
    pub heart: Texture2D,
    pub empty_heart: Texture2D,
    pub background: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        let player_texture = load_scalable_texture("resources/player_sprite.png").await;
        let enemy_texture = load_scalable_texture("resources/enemy_sprite.png").await;

        Self {
            player_atlas: TextureAtlas::from_grid(player_texture, (50., 50.), 4, 5),
            enemy_atlas: TextureAtlas::from_grid(enemy_texture, (24., 24.), 3, 4),
            heart: load_scalable_texture("resources/heart.png").await,
            empty_heart: load_scalable_texture("resources/empty_heart.png").await,
            background: load_scalable_texture("resources/background.png").await,
        }
    }
}

pub struct Animations {
    pub attack_up: Animation,
    pub attack_right: Animation,
    pub attack_bottom: Animation,
    pub attack_left: Animation,
    pub enemy_up: Animation,
    pub enemy_right: Animation,
    pub enemy_bottom: Animation,
    pub enemy_left: Animation,
}

impl Animations {
    pub fn new(textures: &Textures) -> Self {
        let Textures {
            player_atlas,
            enemy_atlas,
            ..
        } = textures;

        Self {
            attack_up: Animation::new(player_atlas.clone(), (4..8).collect(), 0.05, false),
            attack_right: Animation::new(player_atlas.clone(), (8..12).collect(), 0.05, false),
            attack_left: Animation::new(player_atlas.clone(), (12..16).collect(), 0.05, false),
            attack_bottom: Animation::new(player_atlas.clone(), (16..20).collect(), 0.05, false),
            enemy_up: Animation::new(enemy_atlas.clone(), vec![9, 10, 11, 10], 0.1, true),
            enemy_right: Animation::new(enemy_atlas.clone(), vec![6, 7, 8, 7], 0.1, true),
            enemy_left: Animation::new(enemy_atlas.clone(), vec![3, 4, 5, 4], 0.1, true),
            enemy_bottom: Animation::new(enemy_atlas.clone(), vec![0, 1, 2, 1], 0.1, true),
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

    pub fn draw_centered(text: &str, x: f32, y: f32, params: TextParams) {
        let size = measure(text, params);

        let x = x - size.width / 2.;
        let y = y - size.height / 2.;

        draw_text_ex(text, x, y, params);
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

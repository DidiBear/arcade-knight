use std::rc::Rc;

use macroquad::prelude::*;

use crate::{
    animation::{Animation, TextureAtlas},
    direction::Side,
    player::AttackAnimation,
    screen_drawer::load_scalable_texture,
};

pub struct Textures {
    pub player_atlas: Rc<TextureAtlas>,
    pub enemy_atlas: Rc<TextureAtlas>,
    pub heart: Texture2D,
    pub empty_heart: Texture2D,
    pub background: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        let player_texture = load_scalable_texture("resources/player_sprite.png").await;
        let enemy_texture = load_scalable_texture("resources/enemy_sprite.png").await;

        Self {
            player_atlas: Rc::new(TextureAtlas::from_grid(player_texture, (50., 50.), 4, 5)),
            enemy_atlas: Rc::new(TextureAtlas::from_grid(enemy_texture, (24., 24.), 3, 4)),
            heart: load_scalable_texture("resources/heart.png").await,
            empty_heart: load_scalable_texture("resources/empty_heart.png").await,
            background: load_scalable_texture("resources/background.png").await,
        }
    }
}

pub struct Animations {
    player_atlas: Rc<TextureAtlas>,
    enemy_atlas: Rc<TextureAtlas>,
}

impl Animations {
    pub fn new(textures: &Textures) -> Self {
        Self {
            player_atlas: textures.player_atlas.clone(),
            enemy_atlas: textures.enemy_atlas.clone(),
        }
    }

    /// Returns a player attack animation for the given side.   
    pub fn player_attack(&self, side: Side) -> AttackAnimation {
        let indexes = match side {
            Side::Up => 4..8,
            Side::Right => 8..12,
            Side::Left => 12..16,
            Side::Down => 16..20,
        };

        AttackAnimation::new(self.player_atlas.clone(), indexes)
    }

    /// Returns an enemy walking animation for the given side.   
    pub fn enemy_walking(&self, side: Side) -> Animation {
        let indexes = match side {
            Side::Up => vec![9, 10, 11, 10],
            Side::Right => vec![6, 7, 8, 7],
            Side::Left => vec![3, 4, 5, 4],
            Side::Down => vec![0, 1, 2, 1],
        };

        Animation::new(self.enemy_atlas.clone(), indexes, 0.1, true)
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

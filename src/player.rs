use macroquad::prelude::*;

use crate::{
    animation::{Animation, TextureAtlas},
    character::Character,
    direction::{Side, DIRECTION_KEYS, DOWN},
    enemy::Enemy,
    resources::Animations,
    GAME_HEIGHT, GAME_WIDTH,
};

pub struct Player {
    pub character: Character,
    pub attacking: Option<Animation>,
}

impl Player {
    /// Creates the player entity centered in the middle of the screen.
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            character: Character::new(GAME_WIDTH / 2., GAME_HEIGHT / 2., w, h, DOWN),
            attacking: None,
        }
    }

    /// Updates the player's direction depending on the pressed keys.
    pub fn update_direction(&mut self) {
        self.character.direction = DIRECTION_KEYS
            .iter()
            .filter(|(key, _)| is_key_down(*key))
            .map(|(_, direction)| *direction)
            .next()
            .unwrap_or(self.character.direction);
    }

    /// Updates the player's direction depending on the pressed keys.
    pub fn update_animation(&mut self) {
        if let Some(animation) = &mut self.attacking {
            if animation.tick().is_finished() {
                self.attacking = None;
            }
        }
    }

    /// Returns a slash attack positioned at the player's direction   
    pub fn slash_attack(&mut self, animations: &Animations) -> Slash {
        let Character {
            body, direction, ..
        } = self.character;

        let mut animation = match direction.side {
            Side::Up => animations.attack_up.clone(),
            Side::Down => animations.attack_bottom.clone(),
            Side::Left => animations.attack_left.clone(),
            Side::Right => animations.attack_right.clone(),
        };
        animation.restart();
        self.attacking = Some(animation);

        Slash(body.offset(direction.vector * body.size()))
    }

    pub fn draw(&self, player_atlas: &TextureAtlas) {
        let (x, y) = self.character.position();

        if let Some(animation) = &self.attacking {
            animation.draw_current_centered(x, y);
        } else {
            player_atlas.draw_tile_centered(self.character.direction.tile_index, x, y);
        }
    }
}

/// A slash attack.
pub struct Slash(Rect);

impl Slash {
    /// Returns true if the slash attack kill the given character.
    pub fn kill(&self, enemy: &Enemy) -> bool {
        self.0.overlaps(&enemy.character.body)
    }

    pub fn draw(&self) {
        let Rect { x, y, w, h } = self.0;
        draw_rectangle(x, y, w, h, BLUE);
    }
}

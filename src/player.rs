use std::{ops::Range, rc::Rc};

use macroquad::prelude::*;

use crate::{
    animation::{Animation, TextureAtlas},
    character::Character,
    direction::Direction,
    enemy::Enemy,
    resources::{Animations, Textures},
    GAME_HEIGHT, GAME_WIDTH,
};

pub struct Player {
    pub character: Character,
    attacking: Option<AttackAnimation>,
    player_atlas: Rc<TextureAtlas>,
}

impl Player {
    /// Creates the player entity centered in the middle of the screen.
    pub fn new(w: f32, h: f32, textures: &Textures) -> Self {
        Self {
            character: Character::new(GAME_WIDTH / 2., GAME_HEIGHT / 2., w, h, Direction::Down),
            attacking: None,
            player_atlas: textures.player_atlas.clone(),
        }
    }

    /// Updates the player's direction depending on the pressed keys.
    pub fn update_direction(&mut self) {
        if let Some(direction) = Direction::iter().find(|dir| is_key_down(KeyCode::from(*dir))) {
            self.character.direction = direction
        }
    }

    /// Starts the animation of an attack to the current direction.
    pub fn start_attack(&mut self, animations: &Animations) {
        self.attacking = Some(animations.player_attack(self.character.direction));
    }

    /// Updates the animation of the attack.
    pub fn animate_attack(&mut self) {
        if let Some(animation) = &mut self.attacking {
            if animation.0.tick().is_finished() {
                self.attacking = None;
            }
        }
    }

    /// Returns true if the player kills the given enemy with its attack.   
    pub fn kill(&self, enemy: &Enemy) -> bool {
        self.attacking
            .as_ref()
            .filter(|attack| attack.is_attack_frame())
            .map(|_| self.slash_attack())
            .map_or(false, |slash| slash.overlaps(&enemy.character.body))
    }

    pub fn slash_attack(&self) -> Rect {
        let Character {
            body, direction, ..
        } = self.character;

        let mut slash = body;

        match direction {
            Direction::Up | Direction::Down => slash.scale(1., 1.5),
            Direction::Right | Direction::Left => slash.scale(1.5, 1.),
        }



        // let direction = match direction {
        //     Direction::Up | Direction::Left => Vec2::from(direction) * 1.5,
        //     Direction::Down | Direction::Right => Vec2::from(direction),
        // }



        let mut direction = Vec2::from(direction);

        if direction.x < 0. || direction.y < 0. {
            direction *= 1.5;
        }
        slash.offset(direction * body.size())
    }

    /// Draws the player either as idle of in an animation.
    pub fn draw(&self) {
        let (x, y) = self.character.position();

        if let Some(animation) = &self.attacking {
            animation.0.draw_current_centered(x, y);
        } else {
            self.draw_idle(x, y);
        }
    }

    fn draw_idle(&self, x: f32, y: f32) {
        let tile_index = match self.character.direction {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };
        self.player_atlas.draw_tile_centered(tile_index, x, y);
    }
}

pub struct AttackAnimation(Animation);

impl AttackAnimation {
    pub fn new(player_atlas: Rc<TextureAtlas>, indexes: Range<usize>) -> Self {
        Self(Animation::new(player_atlas, indexes.collect(), 0.05, false))
    }

    pub const fn is_attack_frame(&self) -> bool {
        self.0.current_frame == 2 // The third frame is when the sword is the farthest
    }
}

use std::{ops::Range, rc::Rc};

use macroquad::prelude::*;

use crate::{
    animation::{Animation, TextureAtlas},
    character::Character,
    direction::Direction,
    enemy::Enemy,
    resources::Animations,
    GAME_HEIGHT, GAME_WIDTH,
};

pub struct Player {
    pub character: Character,
    pub attacking: Option<AttackAnimation>,
}

impl Player {
    /// Creates the player entity centered in the middle of the screen.
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            character: Character::new(GAME_WIDTH / 2., GAME_HEIGHT / 2., w, h, Direction::Down),
            attacking: None,
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
    pub fn update_attack(&mut self) {
        if let Some(animation) = &mut self.attacking {
            if animation.0.tick().is_finished() {
                self.attacking = None;
            }
        }
    }

    /// Returns true if the player kills the given enemy with its attack.   
    pub fn kill(&self, enemy: &Enemy) -> bool {
        let Character {
            body, direction, ..
        } = self.character;

        self.attacking
            .as_ref()
            .filter(|attack| attack.is_attack_frame())
            .map(|_| body.offset(Vec2::from(direction) * body.size()))
            .map_or(false, |slash| slash.overlaps(&enemy.character.body))
    }

    pub fn draw(&self, player_atlas: &TextureAtlas) {
        let (x, y) = self.character.position();

        if let Some(animation) = &self.attacking {
            animation.0.draw_current_centered(x, y);
        } else {
            let tile_index = idle_player_tile_index(self.character.direction);
            player_atlas.draw_tile_centered(tile_index, x, y);
        }
    }
}

/// Returns the index of the idle player in the texture atlas.
const fn idle_player_tile_index(direction: Direction) -> usize {
    match direction {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    }
}

#[derive(Clone)]
pub struct AttackAnimation(Animation);

impl AttackAnimation {
    pub fn new(player_atlas: Rc<TextureAtlas>, indexes: Range<usize>) -> Self {
        Self(Animation::new(player_atlas, indexes.collect(), 0.05, false))
    }

    pub const fn is_attack_frame(&self) -> bool {
        self.0.current_frame == 2 // The third frame is when the sword is the farthest
    }
}

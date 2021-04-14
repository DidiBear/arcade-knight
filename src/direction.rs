use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[allow(clippy::enum_glob_use)]
use Direction::*;

impl Direction {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Up, Right, Down, Left].iter().copied()
    }
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        let (x, y) = match direction {
            Up => (0., -1.),
            Right => (1., 0.),
            Down => (0., 1.),
            Left => (-1., 0.),
        };

        Self { x, y }
    }
}

impl From<Direction> for KeyCode {
    fn from(direction: Direction) -> Self {
        match direction {
            Up => Self::Up,
            Right => Self::Right,
            Down => Self::Down,
            Left => Self::Left,
        }
    }
}

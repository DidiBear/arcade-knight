use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Side {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
pub struct Direction {
    pub side: Side,
    pub tile_index: usize,
    pub vector: Vec2,
}

pub const UP: Direction = Direction {
    side: Side::Up,
    tile_index: 0,
    vector: Vec2 { x: 0., y: -1. },
};
pub const RIGHT: Direction = Direction {
    side: Side::Right,
    tile_index: 1,
    vector: Vec2 { x: 1., y: 0. },
};
pub const DOWN: Direction = Direction {
    side: Side::Down,
    tile_index: 2,
    vector: Vec2 { x: 0., y: 1. },
};
pub const LEFT: Direction = Direction {
    side: Side::Left,
    tile_index: 3,
    vector: Vec2 { x: -1., y: 0. },
};

pub const DIRECTION_KEYS: [(KeyCode, Direction); 4] = [
    (KeyCode::Up, UP),
    (KeyCode::Down, DOWN),
    (KeyCode::Left, LEFT),
    (KeyCode::Right, RIGHT),
];

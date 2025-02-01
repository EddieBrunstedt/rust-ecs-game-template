use raylib::prelude::*;

use super::PlayerId;

#[derive(Clone)]
pub struct Position(pub Vector2);

#[derive(Clone)]
pub struct Renderable {
    pub color: Color,
    pub size: f32,
}

#[derive(Clone)]
pub struct Player {
    pub id: PlayerId,
    pub is_local: bool,
    pub speed: f32,
}

pub struct Input {
    pub movement: Vector2,

    // Needed later when multiplayer is implemented
    #[allow(dead_code)]
    pub player_id: PlayerId,
}

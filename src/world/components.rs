use raylib::prelude::*;

use super::PlayerId;

pub struct Position(pub Vector2);

pub struct Visual {
    pub color: Color,
    pub size: f32,
}

pub struct Player {
    pub id: PlayerId,
    pub is_local: bool,
    pub speed: f32,
}

pub struct Control {
    pub movement: Vector2,

    // Needed later when multiplayer is implemented
    #[allow(dead_code)]
    pub player_id: PlayerId,
}

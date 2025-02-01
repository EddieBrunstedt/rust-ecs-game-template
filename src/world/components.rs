use raylib::prelude::*;

#[derive(Clone)]
pub struct Position(pub Vector2);

#[derive(Clone)]
pub struct Renderable {
    pub color: Color,
    pub size: f32,
}

#[derive(Clone)]
pub struct Player {
    pub speed: f32,
}

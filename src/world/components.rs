use super::{EntityId, PlayerId};
use raylib::prelude::*;

pub struct PositionComponent(pub Vector2);

pub struct VisualComponent {
    pub color: Color,
    pub size: f32,
}

pub struct PlayerComponent {
    pub id: PlayerId,
    pub is_local: bool,
    pub speed: f32,
}

pub struct ControlComponent {
    pub movement: Vector2,
}

pub struct CameraComponent {
    pub following_entity: Option<EntityId>,
    pub offset: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

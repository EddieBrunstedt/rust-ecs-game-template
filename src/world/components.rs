//! Component definitions for the Entity Component System.

use super::EntityId;
use raylib::prelude::*;

/// Represents an entity's position in 2D space.
pub struct PositionComponent {
    pub coords: Vector2,
}

/// Visual representation properties for an entity.
pub struct VisualComponent {
    /// Color used for rendering
    pub color: Color,
    /// Size of the entity's visual representation
    pub size: f32,
}

/// Identifies an entity as a player and stores player-specific data.
pub struct PlayerComponent {
    /// Whether this player is controlled locally
    pub is_local: bool,
    /// Movement speed multiplier
    pub speed: f32,
}

/// Stores input control state for an entity.
pub struct ControlComponent {
    /// Current movement vector
    pub movement: Vector2,
}

/// Camera properties and behavior configuration.
pub struct CameraComponent {
    /// Entity that the camera should follow (if any)
    pub following_entity: Option<EntityId>,
    /// Camera offset from target position
    pub offset: Vector2,
    /// Camera rotation in degrees
    pub rotation: f32,
    /// Camera zoom level
    pub zoom: f32,
}

/// Tracks the game score
pub struct ScoreComponent {
    /// Current score value
    pub value: i32,
}

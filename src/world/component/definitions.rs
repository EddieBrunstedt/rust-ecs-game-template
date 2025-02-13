use crate::world::component::ComponentMap;
use crate::world::{ComponentStorage, Components, Entity};
use crate::{define_component_storages, define_components, impl_component_storage};
use raylib::prelude::*;

define_components! {
    (camera, CameraComponent),
    (controls, ControlsComponent),
    (player, PlayerComponent),
    (position, PositionComponent),
    (score, ScoreComponent),
    (ui_visibility, UiVisibilityComponent),
    (visual, VisualComponent),
}

/// Camera properties and behavior configuration.
pub struct CameraComponent {
    /// Entity that the camera should follow (if any).
    pub following_entity: Option<Entity>,
    /// Camera offset from target position in screen coordinates.
    pub offset: Vector2,
    /// Camera rotation in degrees, clockwise.
    pub rotation: f32,
    /// Camera zoom level, where 1.0 is default view.
    pub zoom: f32,
}

/// Stores input control state for an entity.
pub struct ControlsComponent {
    /// Current movement vector, normalized between -1 and 1 on each axis.
    pub movement_direction: Vector2,
}

/// Identifies an entity as a player.
/// Contains properties that affect player behavior.
pub struct PlayerComponent {
    /// Movement speed in units per second.
    pub speed: f32,
}

/// Represents an entity's position in 2D space.
/// Used as the primary source of truth for entity locations.
pub struct PositionComponent {
    pub coords: Vector2,
}

/// Tracks the game score.
/// Usually attached to a single entity that manages scoring.
pub struct ScoreComponent {
    /// Current score value.
    pub value: usize,
}

pub struct UiVisibilityComponent {
    pub fps_counter: bool,
    pub press_p_for_paus: bool,
}

/// Visual representation properties for an entity.
/// Defines how an entity should be rendered.
pub struct VisualComponent {
    /// Color used for rendering the entity.
    pub color: Color,
    /// Size of the entity's visual representation in pixels.
    pub size: f32,
}

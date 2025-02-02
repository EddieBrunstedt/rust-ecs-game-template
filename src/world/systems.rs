//! System implementations for the Entity Component System.

use super::EntityId;
use super::World;
use crate::config::GameConfig;
use raylib::prelude::*;

/// Defines behavior for systems that render to the screen.
pub trait RenderSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig);
}

/// Defines behavior for systems that update game state.
pub trait UpdateSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig);
}

/// Updates camera position to follow its target entity.
pub struct CameraFollowSystem;

/// Handles collision detection between entities.
pub struct CollisionDetectionSystem;

/// Processes input and updates entity control states.
pub struct HandleInputSystem;

/// Renders the score on screen
pub struct RenderScoreSystem;

/// Renders all visible entities to the screen.
pub struct RenderVisualsSystem;

/// Updates entity positions based on their control states.
pub struct UpdatePlayerPositionsSystem;

impl UpdateSystem for CameraFollowSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle, _config: &GameConfig) {
        let updates: Vec<_> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let camera = world.camera(*entity)?;
                let target_entity = camera.following_entity?;
                let target_position = world.expect_position(target_entity);
                Some((*entity, target_position.0))
            })
            .collect();

        for (entity, new_position) in updates {
            world.expect_position_mut(entity).0 = new_position;
        }
    }
}

impl UpdateSystem for CollisionDetectionSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle, config: &GameConfig) {
        let score_entity = world
            .entity_ids
            .iter()
            .find(|&&e| world.has_score(e))
            .copied();

        let player_data: Vec<(u64, Vector2, f32)> = world
            .entity_ids
            .iter()
            .filter_map(|&entity| {
                if world.has_player(entity) {
                    let position = world.expect_position(entity);
                    let visual = world.visual(entity)?;
                    Some((entity, position.0, visual.size))
                } else {
                    None
                }
            })
            .collect();

        let mut collision_count = 0;
        let entities_to_remove: Vec<EntityId> = world
            .entity_ids
            .iter()
            .filter_map(|&entity| {
                if world.has_player(entity) {
                    return None;
                }

                let position = world.position(entity)?;
                let visual = world.visual(entity)?;

                for (_player_entity, player_pos, player_size) in &player_data {
                    let distance = (*player_pos - position.0).length();
                    if distance < (player_size + visual.size) {
                        collision_count += config.points_per_collision;
                        return Some(entity);
                    }
                }
                None
            })
            .collect();

        if collision_count > 0 {
            if let Some(score_entity) = score_entity {
                if let Some(score) = world.score_mut(score_entity) {
                    score.value += collision_count;
                }
            }
        }

        for entity in entities_to_remove {
            world.remove_entity(entity);
        }
    }
}

impl UpdateSystem for HandleInputSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle, _config: &GameConfig) {
        let control_updates: Vec<(EntityId, Vector2)> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let player = world.player(*entity)?;
                world.controls(*entity);

                if player.is_local {
                    let mut movement = Vector2::zero();
                    if raylib_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
                        movement.x += 1.0
                    }
                    if raylib_handle.is_key_down(KeyboardKey::KEY_LEFT) {
                        movement.x -= 1.0
                    }
                    if raylib_handle.is_key_down(KeyboardKey::KEY_DOWN) {
                        movement.y += 1.0
                    }
                    if raylib_handle.is_key_down(KeyboardKey::KEY_UP) {
                        movement.y -= 1.0
                    }

                    Some((*entity, movement))
                } else {
                    todo!("Implement movement for network players");
                }
            })
            .collect();

        for (entity, movement) in control_updates {
            world.expect_controls_mut(entity).movement = movement.normalized();
        }
    }
}

impl RenderSystem for RenderScoreSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        if let Some(score) = world.entity_ids.iter().find_map(|e| world.score(*e)) {
            draw_handle.draw_text(
                &format!("Score: {}", score.value),
                config.score_text_x,
                config.score_text_y,
                config.score_font_size,
                config.score_text_color,
            );
        }
    }
}

impl RenderSystem for RenderVisualsSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, _config: &GameConfig) {
        let camera_data = world
            .entity_ids
            .iter()
            .find_map(|entity| {
                let camera = world.camera(*entity)?;
                let position = world.expect_position(*entity);
                Some((camera, position))
            })
            .expect("No camera entity found in world");

        let (camera_component, camera_position) = camera_data;

        let camera = Camera2D {
            target: camera_position.0,
            offset: camera_component.offset,
            rotation: camera_component.rotation,
            zoom: camera_component.zoom,
        };

        let mut camera_draw = draw_handle.begin_mode2D(camera);

        for entity in &world.entity_ids {
            if let (Some(position), Some(visual)) = (world.position(*entity), world.visual(*entity))
            {
                camera_draw.draw_circle(
                    position.0.x as i32,
                    position.0.y as i32,
                    visual.size,
                    visual.color,
                );
            }
        }
    }
}

impl UpdateSystem for UpdatePlayerPositionsSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle, _config: &GameConfig) {
        let updates: Vec<(EntityId, Vector2)> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let player = world.player(*entity)?;
                let control = world.expect_controls(*entity);
                let position = world.expect_position(*entity);
                let new_position = position.0 + (control.movement * player.speed);
                Some((*entity, new_position))
            })
            .collect();

        for (entity, new_position) in updates {
            world.expect_position_mut(entity).0 = new_position;
        }
    }
}

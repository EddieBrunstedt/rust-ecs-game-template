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
                Some((*entity, target_position.coords))
            })
            .collect();

        for (entity, new_position) in updates {
            world.expect_position_mut(entity).coords = new_position;
        }
    }
}

impl UpdateSystem for CollisionDetectionSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle, config: &GameConfig) {
        let score_entity = world.entity_ids.iter().find(|&&e| world.has_score(e));

        let player_data: Vec<_> = world
            .entity_ids
            .iter()
            .filter(|&&e| world.has_player(e))
            .filter_map(|&e| Some((e, world.expect_position(e).coords, world.visual(e)?.size)))
            .collect();

        let entities_to_remove: Vec<_> = world
            .entity_ids
            .iter()
            .filter(|&&e| !world.has_player(e))
            .filter_map(|&e| {
                let pos = world.position(e)?;
                let size = world.visual(e)?.size;

                player_data
                    .iter()
                    .any(|(_, p_pos, p_size)| (*p_pos - pos.coords).length() < (*p_size + size))
                    .then_some(e)
            })
            .collect();

        let collision_count = (entities_to_remove.len() as i32) * config.points_per_collision;

        if collision_count > 0 {
            if let Some(&e) = score_entity {
                if let Some(score) = world.score_mut(e) {
                    score.value += collision_count;
                }
            }
        }

        entities_to_remove
            .iter()
            .for_each(|&e| world.remove_entity(e));
    }
}

impl UpdateSystem for HandleInputSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig) {
        let control_updates: Vec<(EntityId, Vector2)> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                world.player(*entity)?;
                world.expect_controls(*entity);

                let movement = Vector2 {
                    x: raylib_handle.is_key_down(config.key_move_right) as i32 as f32
                        - raylib_handle.is_key_down(config.key_move_left) as i32 as f32,
                    y: raylib_handle.is_key_down(config.key_move_down) as i32 as f32
                        - raylib_handle.is_key_down(config.key_move_up) as i32 as f32,
                };

                Some((*entity, movement))
            })
            .collect();

        for (entity, movement) in control_updates {
            let movement = if config.normalize_movement {
                movement.normalized()
            } else {
                movement
            };
            world.expect_controls_mut(entity).movement = movement;
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
        let (camera, position) = world
            .entity_ids
            .iter()
            .find_map(|&id| Some((world.camera(id)?, world.expect_position(id))))
            .expect("No camera entity found in world");

        let mut draw = draw_handle.begin_mode2D(Camera2D {
            target: position.coords,
            offset: camera.offset,
            rotation: camera.rotation,
            zoom: camera.zoom,
        });

        world
            .entity_ids
            .iter()
            .filter_map(|&id| Some((world.position(id)?, world.visual(id)?)))
            .for_each(|(position, visual)| {
                draw.draw_circle(
                    position.coords.x as i32,
                    position.coords.y as i32,
                    visual.size,
                    visual.color,
                )
            });
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
                let new_position = position.coords + (control.movement * player.speed);
                Some((*entity, new_position))
            })
            .collect();

        for (entity, new_position) in updates {
            world.expect_position_mut(entity).coords = new_position;
        }
    }
}

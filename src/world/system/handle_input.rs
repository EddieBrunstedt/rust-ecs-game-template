use super::UpdateSystem;
use crate::{
    config::GameConfig,
    world::{EntityId, World},
};
use raylib::prelude::*;

/// Processes input and updates entity control states.
pub struct HandleInputSystem;

impl UpdateSystem for HandleInputSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig) {
        let control_updates: Vec<(EntityId, Vector2)> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                world.components.player(*entity)?;
                world.components.expect_controls(*entity);

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
            world.components.expect_controls_mut(entity).movement = movement;
        }
    }
}

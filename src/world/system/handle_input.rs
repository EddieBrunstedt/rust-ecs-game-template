use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{Entity, SceneTransition, World},
};

use super::UpdateSystem;

/// Processes input and updates entity control states.
pub struct HandleInputSystem;

impl UpdateSystem for HandleInputSystem {
    fn run(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
        let control_updates: Vec<(Entity, Vector2)> = world
            .iter_entities()
            .filter_map(|entity| {
                world.components.player(entity)?;
                world.components.expect_controls(entity);

                let movement = Vector2 {
                    x: raylib_handle.is_key_down(config.key_move_right) as i32 as f32
                        - raylib_handle.is_key_down(config.key_move_left) as i32 as f32,
                    y: raylib_handle.is_key_down(config.key_move_down) as i32 as f32
                        - raylib_handle.is_key_down(config.key_move_up) as i32 as f32,
                };

                Some((entity, movement))
            })
            .collect();

        for (entity, movement) in control_updates {
            let movement = if config.normalize_movement {
                movement.normalized()
            } else {
                movement
            };

            world
                .components
                .expect_controls_mut(entity)
                .movement_direction = movement;
        }

        None
    }
}

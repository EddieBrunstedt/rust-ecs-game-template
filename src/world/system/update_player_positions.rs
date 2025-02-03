use super::UpdateSystem;
use crate::{
    config::GameConfig,
    world::{GenerationalId, World},
};
use raylib::prelude::*;

/// Updates entity positions based on their control states.
pub struct UpdatePlayerPositionsSystem;

impl UpdateSystem for UpdatePlayerPositionsSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle, _config: &GameConfig) {
        let delta_time = raylib_handle.get_frame_time();

        // Collect all active entities that need position updates
        let updates: Vec<(GenerationalId, Vector2)> = world
            .iter_entities()
            .filter_map(|entity| {
                let player = world.components.player(entity)?;
                let control = world.components.expect_controls(entity);
                let position = world.components.expect_position(entity);
                let frame_movement = control
                    .movement_direction
                    .scale_by(player.speed * delta_time);
                let new_position = position.coords + frame_movement;
                Some((entity, new_position))
            })
            .collect();

        // Apply the position updates
        for (entity, new_position) in updates {
            if world.is_entity_alive(entity) {
                world.components.expect_position_mut(entity).coords = new_position;
            }
        }
    }
}

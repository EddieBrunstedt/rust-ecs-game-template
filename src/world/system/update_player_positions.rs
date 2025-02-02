use super::UpdateSystem;
use crate::{
    config::GameConfig,
    world::{EntityId, World},
};
use raylib::prelude::*;

/// Updates entity positions based on their control states.
pub struct UpdatePlayerPositionsSystem;

impl UpdateSystem for UpdatePlayerPositionsSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle, _config: &GameConfig) {
        let updates: Vec<(EntityId, Vector2)> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let player = world.components.player(*entity)?;
                let control = world.components.expect_controls(*entity);
                let position = world.components.expect_position(*entity);
                let new_position = position.coords + (control.movement * player.speed);
                Some((*entity, new_position))
            })
            .collect();

        for (entity, new_position) in updates {
            world.components.expect_position_mut(entity).coords = new_position;
        }
    }
}

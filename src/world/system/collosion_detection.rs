use super::UpdateSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Handles collision detection between entities.
pub struct CollisionDetectionSystem;

impl UpdateSystem for CollisionDetectionSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle, config: &GameConfig) {
        let score_entity = world
            .iter_entities()
            .find(|&e| world.components.has_score(e));

        let player_data: Vec<_> = world
            .iter_entities()
            .filter(|&e| world.components.has_player(e))
            .filter_map(|e| {
                Some((
                    e,
                    world.components.expect_position(e).coords,
                    world.components.visual(e)?.size,
                ))
            })
            .collect();

        let entities_to_remove: Vec<_> = world
            .iter_entities()
            .filter(|&e| !world.components.has_player(e))
            .filter_map(|e| {
                let pos = world.components.position(e)?;
                let size = world.components.visual(e)?.size;

                player_data
                    .iter()
                    .any(|(_, p_pos, p_size)| (*p_pos - pos.coords).length() < (*p_size + size))
                    .then_some(e)
            })
            .collect();

        // TODO: Maybe change collision count to usize?
        let collision_count = (entities_to_remove.len() as i32) * config.points_per_collision;

        if collision_count > 0 {
            if let Some(e) = score_entity {
                if let Some(score) = world.components.score_mut(e) {
                    score.value += collision_count;
                }
            }
        }

        entities_to_remove
            .iter()
            .for_each(|&e| world.remove_entity(e));
    }
}

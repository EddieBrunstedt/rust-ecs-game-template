use super::UpdateSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Updates camera position to follow its target entity.
pub struct CameraFollowSystem;

impl UpdateSystem for CameraFollowSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle, _config: &GameConfig) {
        let updates: Vec<_> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let camera = world.components.camera(*entity)?;
                let target_entity = camera.following_entity?;
                let target_position = world.components.expect_position(target_entity);
                Some((*entity, target_position.coords))
            })
            .collect();

        for (entity, new_position) in updates {
            world.components.expect_position_mut(entity).coords = new_position;
        }
    }
}

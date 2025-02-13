use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{SceneTransition, World},
};

use super::UpdateSystem;

/// Updates camera position to follow its target entity.
pub struct CameraFollowSystem;

impl UpdateSystem for CameraFollowSystem {
    fn run(
        &self,
        world: &mut World,
        _raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        let updates: Vec<_> = world
            .iter_entities()
            .filter_map(|entity| {
                let camera = world.components.camera(entity)?;
                let target_entity = camera.following_entity?;
                let target_position = world.components.expect_position(target_entity);
                Some((entity, target_position.coords))
            })
            .collect();

        for (entity, new_position) in updates {
            world.components.expect_position_mut(entity).coords = new_position;
        }

        None
    }
}

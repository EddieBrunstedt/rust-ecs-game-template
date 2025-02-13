use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{SceneTransition, World},
};

use super::UpdateSystem;

/// Responsible for tracking game time progression.
/// Updates the global GameTime resource with frame delta time and total elapsed time.
pub struct UpdateTimeSystem;

impl UpdateSystem for UpdateTimeSystem {
    fn run(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if let Some(game_time) = world.resources.game_time_mut() {
            game_time.total += raylib_handle.get_frame_time();
        }

        None
    }
}

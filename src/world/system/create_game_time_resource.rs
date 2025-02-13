use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{resource::*, SceneTransition, World},
};

use super::UpdateSystem;

pub struct CreateGameTimeResourceSystem;

impl UpdateSystem for CreateGameTimeResourceSystem {
    fn run(
        &self,
        world: &mut World,
        _raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        world
            .resources
            .insert_game_time(GameTimeResource { total: 0.0 });

        None
    }
}

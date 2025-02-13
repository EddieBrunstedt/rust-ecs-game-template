use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{component::*, SceneTransition, World},
};

use super::UpdateSystem;

pub struct CreateScore;

impl UpdateSystem for CreateScore {
    fn run(
        &self,
        world: &mut World,
        _raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        world.build_entity().with(ScoreComponent { value: 0 });
        None
    }
}

use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{component::*, SceneTransition, World},
};

use super::UpdateSystem;

pub struct CreateUiVisibility;

impl UpdateSystem for CreateUiVisibility {
    fn run(
        &self,
        world: &mut World,
        _raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
        world.build_entity().with(UiVisibilityComponent {
            fps_counter: config.fps_initial_visibility,
            press_p_for_paus: true,
        });

        None
    }
}

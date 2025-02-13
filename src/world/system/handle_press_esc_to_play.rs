use raylib::prelude::*;

use crate::{config::GameConfig, world::*};

use super::UpdateSystem;

/// Processes input and updates entity control states.
pub struct HandlePressEscToPlay;

impl UpdateSystem for HandlePressEscToPlay {
    fn run(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            let ui_entity = world
                .iter_entities()
                .find(|e| world.components.ui_visibility(*e).is_some())
                .expect("UI visibility state is required");

            world
                .components
                .expect_ui_visibility_mut(ui_entity)
                .press_p_for_paus = true;

            Some(SceneTransition::Pop)
        } else {
            None
        }
    }
}

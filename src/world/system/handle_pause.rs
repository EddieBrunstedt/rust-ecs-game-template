use raylib::prelude::*;

use crate::{config::GameConfig, world::*};

use super::UpdateSystem;

/// Processes input and updates entity control states.
pub struct HandlePauseSystem;

impl UpdateSystem for HandlePauseSystem {
    fn run(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_P) {
            let ui_entity = world
                .iter_entities()
                .find(|e| world.components.ui_visibility(*e).is_some())
                .expect("UI visibility state is required");

            world
                .components
                .expect_ui_visibility_mut(ui_entity)
                .press_p_for_paus = false;

            Some(SceneTransition::Push(GameScene::Paused(PauseScene)))
        } else {
            None
        }
    }
}

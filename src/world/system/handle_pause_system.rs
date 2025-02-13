use raylib::prelude::*;

use crate::{config::GameConfig, world::*};

use super::UpdateSystem;

/// Processes input and updates entity control states.
pub struct HandlePauseSystem;

impl UpdateSystem for HandlePauseSystem {
    fn run(
        &self,
        _world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_P) {
            Some(SceneTransition::Push(GameScene::Paused(PauseScene)))
        } else {
            None
        }
    }
}

use raylib::prelude::*;

use crate::{config::GameConfig, world::*};

use super::UpdateSystem;

/// Processes input and updates entity control states.
pub struct HandlePressEnterToPlay;

impl UpdateSystem for HandlePressEnterToPlay {
    fn run(
        &self,
        _world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
            Some(SceneTransition::Replace(GameScene::Playing(PlayingScene)))
        } else {
            None
        }
    }
}

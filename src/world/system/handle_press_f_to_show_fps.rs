use raylib::prelude::*;

use crate::{config::GameConfig, world::*};

use super::UpdateSystem;

/// Processes input and updates entity control states.
pub struct HandlePressFToShowFpsSystem;

impl UpdateSystem for HandlePressFToShowFpsSystem {
    fn run(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_F) {
            let entities: Vec<_> = world.iter_entities().collect();

            for entity in entities {
                if let Some(ui_visibility) = world.components.ui_visibility_mut(entity) {
                    ui_visibility.fps_counter = !ui_visibility.fps_counter;
                    break;
                }
            }
        }

        None
    }
}

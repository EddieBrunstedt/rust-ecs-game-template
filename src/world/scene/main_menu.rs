use super::{Scene, SceneTransition};
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

pub struct MainMenuScene;

impl Scene for MainMenuScene {
    fn enter(&mut self, _world: &mut World, _config: &GameConfig) {
        // Setup pause-specific UI entities only
    }

    fn exit(&mut self, _world: &mut World) {
        // Cleanup pause-specific UI entities only
    }

    fn is_transparent(&self) -> bool {
        true // Allow the game scene to render underneath
    }

    fn update(
        &mut self,
        _world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            Some(SceneTransition::Pop) // Return to previous scene
        } else {
            None
        }
    }

    fn render(
        &mut self,
        _world: &mut World,
        _draw_handle: &mut RaylibDrawHandle,
        _config: &GameConfig,
    ) {
        // Run Render Systems
    }
}

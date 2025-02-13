use super::{GameScene, PlayingScene, Scene, SceneTransition};
use crate::{
    config::GameConfig,
    world::{RenderMainMenu, RenderSystem, World},
};
use raylib::prelude::*;

#[derive(Debug)]
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
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
            Some(SceneTransition::Replace(GameScene::Playing(PlayingScene)))
        } else {
            None
        }
    }

    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        RenderMainMenu::run(world, draw_handle, config);
    }
}

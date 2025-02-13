use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{
        GameSystem, HandlePressEnterToPlay, RenderMainMenu, RenderSystem, UpdateSystem, World,
    },
};

use super::{Scene, SceneTransition};

#[derive(Debug)]
pub struct MainMenuScene;

impl Scene for MainMenuScene {
    fn enter(&mut self, _world: &mut World, _config: &GameConfig) {}

    fn exit(&mut self, _world: &mut World) {}

    fn is_transparent(&self) -> bool {
        false
    }

    fn update(
        &mut self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
        let systems_to_run = [GameSystem::HandlePressEnterToPlay(HandlePressEnterToPlay)];

        for system in systems_to_run {
            if let Some(transition) = system.run(world, raylib_handle, config) {
                return Some(transition);
            }
        }

        None
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

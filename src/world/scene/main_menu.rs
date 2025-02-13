use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{GameUpdateSystem, HandlePressEnterToPlaySystem, RenderMainMenu, RenderSystem, World},
};

use super::{Scene, SceneTransition};

pub struct MainMenuScene;

impl Scene for MainMenuScene {
    fn enter(&mut self, _world: &mut World, _config: &GameConfig, _raylib_handle: &RaylibHandle) {}

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
        let systems_to_run = [GameUpdateSystem::HandlePressEnterToPlay(
            HandlePressEnterToPlaySystem,
        )];

        self.run_systems_until_first_transition(world, raylib_handle, config, &systems_to_run)
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

use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{system::*, *},
};

use super::{Scene, SceneTransition};

pub struct PauseScene;

impl Scene for PauseScene {
    fn enter(&mut self, _world: &mut World, _config: &GameConfig) {}

    fn exit(&mut self, _world: &mut World) {}

    fn is_transparent(&self) -> bool {
        true
    }

    fn update(
        &mut self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
        let systems_to_run = [
            GameUpdateSystem::HandlePressEscToPlay(HandlePressEscToPlaySystem),
            GameUpdateSystem::HandlePressFToShowFps(HandlePressFToShowFpsSystem),
        ];

        self.run_systems_until_first_transition(world, raylib_handle, config, &systems_to_run)
    }

    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        RenderPausMenu::run(world, draw_handle, config);
    }
}

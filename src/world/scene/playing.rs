use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{system::*, *},
};

use super::{Scene, SceneTransition};

pub struct PlayingScene;

impl Scene for PlayingScene {
    fn enter(&mut self, world: &mut World, config: &GameConfig, raylib_handle: &RaylibHandle) {
        let systems_to_run = [
            GameUpdateSystem::SpawnPlayerAndCamera(SpawnPlayerAndCameraSystem),
            GameUpdateSystem::CreateGameTimeResourceSystem(CreateGameTimeResourceSystem),
            GameUpdateSystem::SpawnSpiralDots(SpawnSpiralDots),
            GameUpdateSystem::CreateScore(CreateScore),
            GameUpdateSystem::CreateUiVisibility(CreateUiVisibility),
        ];

        self.run_systems_until_first_transition(world, raylib_handle, config, &systems_to_run);
    }

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
        let systems_to_run = [
            GameUpdateSystem::HandlePause(HandlePauseSystem),
            GameUpdateSystem::UpdateTime(UpdateTimeSystem),
            GameUpdateSystem::HandleInput(HandleInputSystem),
            GameUpdateSystem::UpdatePlayerPositions(UpdatePlayerPositionsSystem),
            GameUpdateSystem::CollisionDetection(CollisionDetectionSystem),
            GameUpdateSystem::CameraFollow(CameraFollowSystem),
        ];

        self.run_systems_until_first_transition(world, raylib_handle, config, &systems_to_run)
    }

    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        RenderVisualsSystem::run(world, draw_handle, config);
        RenderScoreSystem::run(world, draw_handle, config);
        RenderGameTimeSystem::run(world, draw_handle, config);
        RenderFpsSystem::run(world, draw_handle, config);
        RenderPressPForPausSystem::run(world, draw_handle, config);
    }
}

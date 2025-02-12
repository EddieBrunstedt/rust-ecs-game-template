use crate::{
    config::GameConfig,
    world::{system::*, World},
};
use raylib::prelude::*;

#[allow(dead_code)]
pub trait Scene {
    fn enter(&mut self, world: &mut World);
    fn exit(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig);
    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    );
}

pub struct MainMenuScene;
pub struct PlayingScene;
pub struct PauseScene;

// Enum to hold all possible scenes
#[allow(dead_code)]
pub enum GameScene {
    MainMenu(MainMenuScene),
    Playing(PlayingScene),
    Paused(PauseScene),
}

impl Scene for PlayingScene {
    fn enter(&mut self, _world: &mut World) {
        // Setup scene-specific entities
    }

    fn exit(&mut self, _world: &mut World) {
        // Cleanup scene-specific entities
    }

    fn update(&mut self, world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig) {
        UpdateTimeSystem::run(world, raylib_handle, config);
        HandleInputSystem::run(world, raylib_handle, config);
        UpdatePlayerPositionsSystem::run(world, raylib_handle, config);
        CollisionDetectionSystem::run(world, raylib_handle, config);
        CameraFollowSystem::run(world, raylib_handle, config);
    }

    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        ClearScreenSystem::run(world, draw_handle, config);
        RenderVisualsSystem::run(world, draw_handle, config);
        RenderScoreSystem::run(world, draw_handle, config);
        RenderGameTimeSystem::run(world, draw_handle, config);
        RenderFpsSystem::run(world, draw_handle, config);
    }
}

impl Scene for MainMenuScene {
    fn enter(&mut self, _world: &mut World) {
        // Setup scene-specific entities
    }

    fn exit(&mut self, _world: &mut World) {
        // Cleanup scene-specific entities
    }

    fn update(&mut self, _world: &mut World, _raylib_handle: &RaylibHandle, _config: &GameConfig) {
        // Run Update Systems
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

impl Scene for PauseScene {
    fn enter(&mut self, _world: &mut World) {
        // Setup scene-specific entities
    }

    fn exit(&mut self, _world: &mut World) {
        // Cleanup scene-specific entities
    }

    fn update(&mut self, _world: &mut World, _raylib_handle: &RaylibHandle, _config: &GameConfig) {
        // Run Update Systems
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

// impl MainMenuScene {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
//
// impl PauseScene {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

impl GameScene {
    pub fn enter(&mut self, world: &mut World) {
        match self {
            GameScene::MainMenu(scene) => scene.enter(world),
            GameScene::Playing(scene) => scene.enter(world),
            GameScene::Paused(scene) => scene.enter(world),
        }
    }

    pub fn exit(&mut self, world: &mut World) {
        match self {
            GameScene::MainMenu(scene) => scene.exit(world),
            GameScene::Playing(scene) => scene.exit(world),
            GameScene::Paused(scene) => scene.exit(world),
        }
    }

    pub fn update(&mut self, world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig) {
        match self {
            GameScene::MainMenu(scene) => scene.update(world, raylib_handle, config),
            GameScene::Playing(scene) => scene.update(world, raylib_handle, config),
            GameScene::Paused(scene) => scene.update(world, raylib_handle, config),
        }
    }

    pub fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        match self {
            GameScene::MainMenu(scene) => scene.render(world, draw_handle, config),
            GameScene::Playing(scene) => scene.render(world, draw_handle, config),
            GameScene::Paused(scene) => scene.render(world, draw_handle, config),
        }
    }
}

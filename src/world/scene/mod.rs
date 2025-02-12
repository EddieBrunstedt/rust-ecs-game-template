use crate::{
    config::GameConfig,
    world::{system::*, World},
};
use raylib::prelude::*;

pub enum SceneTransition {
    /// Replace the current scene entirely, cleaning up all entities
    Replace(GameScene),
    /// Push a new scene on top, preserving the current scene underneath
    Push(GameScene),
    /// Pop the current scene, returning to the previous scene
    Pop,
}

pub trait Scene {
    fn enter(&mut self, world: &mut World);
    fn exit(&mut self, world: &mut World);
    fn update(
        &mut self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition>;
    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    );

    /// Whether this scene should render scenes below it
    fn is_transparent(&self) -> bool {
        false
    }
}

pub struct MainMenuScene;
pub struct PlayingScene;
pub struct PauseScene;

// Enum to hold all possible scenes
pub enum GameScene {
    MainMenu(MainMenuScene),
    Playing(PlayingScene),
    Paused(PauseScene),
}

impl Scene for PlayingScene {
    fn enter(&mut self, _world: &mut World) {
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
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
        UpdateTimeSystem::run(world, raylib_handle, config);
        HandleInputSystem::run(world, raylib_handle, config);
        UpdatePlayerPositionsSystem::run(world, raylib_handle, config);
        CollisionDetectionSystem::run(world, raylib_handle, config);
        CameraFollowSystem::run(world, raylib_handle, config);

        None
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

impl Scene for PauseScene {
    fn enter(&mut self, _world: &mut World) {
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
            Some(SceneTransition::Pop)
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

    pub fn update(
        &mut self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
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

    pub fn is_transparent(&self) -> bool {
        match self {
            GameScene::MainMenu(scene) => scene.is_transparent(),
            GameScene::Playing(scene) => scene.is_transparent(),
            GameScene::Paused(scene) => scene.is_transparent(),
        }
    }
}

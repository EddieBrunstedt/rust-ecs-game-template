mod main_menu;
mod manager;
mod paus;
mod playing;

use raylib::prelude::*;

use crate::{config::GameConfig, world::*};

pub use main_menu::MainMenuScene;
pub use manager::SceneManager;
pub use paus::PauseScene;
pub use playing::PlayingScene;

pub enum SceneTransition {
    /// Replace the current scene entirely, cleaning up all entities
    Replace(GameScene),
    /// Push a new scene on top, preserving the current scene underneath
    Push(GameScene),
    /// Pop the current scene, returning to the previous scene
    Pop,
}

pub trait Scene {
    fn enter(&mut self, world: &mut World, config: &GameConfig);
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
    fn is_transparent(&self) -> bool;

    fn run_systems_until_first_transition(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
        systems_to_run: &[GameUpdateSystem],
    ) -> Option<SceneTransition> {
        for system in systems_to_run {
            if let Some(transition) = system.run(world, raylib_handle, config) {
                return Some(transition);
            }
        }

        None
    }
}

// Enum to hold all possible scenes
pub enum GameScene {
    MainMenu(MainMenuScene),
    Playing(PlayingScene),
    Paused(PauseScene),
}

impl GameScene {
    pub fn enter(&mut self, world: &mut World, config: &GameConfig) {
        match self {
            GameScene::MainMenu(scene) => scene.enter(world, config),
            GameScene::Playing(scene) => scene.enter(world, config),
            GameScene::Paused(scene) => scene.enter(world, config),
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

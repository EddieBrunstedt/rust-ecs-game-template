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

/// Represents a game scene with lifecycle methods for managing state and rendering.
///
/// A scene is a distinct state of the game (e.g., main menu, gameplay, pause screen)
/// that has its own update and render logic. Scenes can be stacked and transitioned
/// between to create complex game state management.
pub trait Scene {
    /// Called when this scene becomes active, either by being pushed onto the stack
    /// or when it becomes the new top scene after a pop operation.
    fn enter(&mut self, world: &mut World, config: &GameConfig);

    /// Called when this scene is being removed from the scene stack.
    fn exit(&mut self, world: &mut World);

    /// Updates the scene's state and handles input.
    ///
    /// # Returns
    /// * `Some(SceneTransition)` if a scene transition should occur
    /// * `None` if no transition is needed
    fn update(
        &mut self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition>;

    /// Renders the scene's current state to the screen.
    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    );

    /// Determines if scenes below this one in the stack should be rendered.
    ///
    /// # Returns
    /// * `true` if lower scenes should be rendered (e.g., for overlay menus)
    /// * `false` if this scene fully obscures lower scenes
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

/// Represents the different ways scenes can transition between each other.
///
/// Used by the scene manager to handle scene stack modifications when scenes
/// request state changes.
pub enum SceneTransition {
    /// Removes the current scene and adds the new scene in its place
    Replace(GameScene),
    /// Adds a new scene on top of the current scene, preserving the current scene below
    Push(GameScene),
    /// Removes the current scene, revealing the scene below (if any)
    Pop,
}

/// Represents all possible scene types in the game.
///
/// This enum serves as a type-safe way to manage different scene implementations
/// while providing a unified interface through trait delegation.
pub enum GameScene {
    /// The main menu scene shown when the game starts
    MainMenu(MainMenuScene),
    /// The active gameplay scene
    Playing(PlayingScene),
    /// The pause menu overlay scene
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

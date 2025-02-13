//! Core game loop and state management.

use crate::{config::GameConfig, world::*};
use raylib::prelude::*;

/// Main game state container and controller.
pub struct Game {
    config: GameConfig,
    raylib_handle: RaylibHandle,
    raylib_thread: RaylibThread,
    scene_manager: SceneManager,
    world: World,
}

/// Builder pattern implementation for game initialization.
/// Handles the setup of core game systems and entities.
pub struct GameBuilder {
    world: World,
    config: GameConfig,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            config: GameConfig::default(),
        }
    }

    pub fn enter(mut self) -> Game {
        let (mut raylib_handle, raylib_thread) = raylib::init()
            .size(self.config.window_width, self.config.window_height)
            .title(self.config.window_title)
            .build();

        raylib_handle.set_exit_key(None);

        let mut scene_manager = SceneManager::new();

        scene_manager.handle_transition(
            SceneTransition::Replace(GameScene::MainMenu(MainMenuScene)),
            &mut self.world,
            &self.config,
        );

        Game {
            config: self.config,
            raylib_handle,
            raylib_thread,
            scene_manager,
            world: self.world,
        }
    }
}

impl Game {
    pub fn run(&mut self) {
        while !self.raylib_handle.window_should_close() {
            // Handle scene updates
            if let Some(current_scene) = self.scene_manager.current_scene() {
                if let Some(transition) =
                    current_scene.update(&mut self.world, &self.raylib_handle, &self.config)
                {
                    self.scene_manager
                        .handle_transition(transition, &mut self.world, &self.config);
                    continue;
                }
            }

            // Begin drawing
            let mut draw_handle = self.raylib_handle.begin_drawing(&self.raylib_thread);
            ClearScreenSystem::run(&mut self.world, &mut draw_handle, &self.config);

            self.scene_manager
                .render(&mut self.world, &mut draw_handle, &self.config);
        }
    }
}

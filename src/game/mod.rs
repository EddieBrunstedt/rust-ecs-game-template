//! Core game loop and state management.

use raylib::prelude::*;

use crate::{config::GameConfig, world::*};

/// Main game state container and controller.
pub struct Game {
    config: GameConfig,
    raylib_handle: RaylibHandle,
    raylib_thread: RaylibThread,
    scene_manager: SceneManager,
    world: World,
}

impl Game {
    /// Creates a new Game instance with default configuration.
    pub fn new() -> Self {
        let config = GameConfig::default();
        let world = World::new();

        let (mut raylib_handle, raylib_thread) = raylib::init()
            .size(config.window_width, config.window_height)
            .title(config.window_title)
            .build();

        raylib_handle.set_exit_key(None);

        Self {
            config,
            raylib_handle,
            raylib_thread,
            scene_manager: SceneManager::with_scene(GameScene::MainMenu(MainMenuScene)),
            world,
        }
    }

    /// Starts the main game loop and runs until the window is closed.
    pub fn run(&mut self) {
        while !self.raylib_handle.window_should_close() {
            // Handle scene updates and transitions
            if self.update_current_scene() {
                // Scene transition occurred, skip rendering this frame
                continue;
            }

            // Render the current frame
            self.render_frame();
        }
    }

    /// Updates the current scene and handles any requested scene transitions.
    ///
    /// # Returns
    /// * `true` if a scene transition occurred
    /// * `false` if no transition happened
    fn update_current_scene(&mut self) -> bool {
        let current_scene = match self.scene_manager.current_scene() {
            Some(scene) => scene,
            None => return false,
        };

        // Check for scene transitions
        if let Some(transition) =
            current_scene.update(&mut self.world, &self.raylib_handle, &self.config)
        {
            self.scene_manager
                .handle_transition(transition, &mut self.world, &self.config);
            return true;
        }

        false
    }

    /// Renders the current frame, including all visible scenes in the stack.
    fn render_frame(&mut self) {
        let mut draw_handle = self.raylib_handle.begin_drawing(&self.raylib_thread);

        // Clear the screen and render the current scene scene stack
        ClearScreenSystem::run(&mut self.world, &mut draw_handle, &self.config);
        self.scene_manager
            .render(&mut self.world, &mut draw_handle, &self.config);
    }
}

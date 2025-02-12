//! Core game loop and state management.

use crate::{config::GameConfig, world::*};
use raylib::prelude::*;

/// Main game state container and controller.
pub struct Game {
    config: GameConfig,
    raylib_handle: RaylibHandle,
    raylib_thread: RaylibThread,
    scene_stack: Vec<GameScene>,
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

    pub fn enter(self) -> Game {
        let (mut raylib_handle, raylib_thread) = raylib::init()
            .size(self.config.window_width, self.config.window_height)
            .title(self.config.window_title)
            .build();

        raylib_handle.set_exit_key(None);

        let mut game = Game {
            config: self.config,
            raylib_handle,
            raylib_thread,
            scene_stack: vec![],
            world: self.world,
        };

        game.handle_scene_transition(SceneTransition::Push(GameScene::Playing(PlayingScene)));

        game
    }
}

impl Game {
    pub fn handle_scene_transition(&mut self, transition: SceneTransition) {
        match transition {
            SceneTransition::Replace(new_scene) => {
                if let Some(mut current_scene) = self.scene_stack.pop() {
                    current_scene.exit(&mut self.world);
                }
                self.scene_stack.push(new_scene);
                self.scene_stack
                    .last_mut()
                    .unwrap()
                    .enter(&mut self.world, &self.config);
            }
            SceneTransition::Push(new_scene) => {
                self.scene_stack.push(new_scene);
                self.scene_stack
                    .last_mut()
                    .unwrap()
                    .enter(&mut self.world, &self.config);
            }
            SceneTransition::Pop => {
                if let Some(mut current_scene) = self.scene_stack.pop() {
                    current_scene.exit(&mut self.world);
                }
            }
        }
    }

    pub fn run(&mut self) {
        while !self.raylib_handle.window_should_close() {
            if let Some(current_scene) = self.scene_stack.last_mut() {
                if let Some(transition) =
                    current_scene.update(&mut self.world, &self.raylib_handle, &self.config)
                {
                    self.handle_scene_transition(transition);
                    continue;
                }
            }

            let mut draw_handle = self.raylib_handle.begin_drawing(&self.raylib_thread);
            for scene in self.scene_stack.iter_mut() {
                scene.render(&mut self.world, &mut draw_handle, &self.config);
            }
        }
    }
}

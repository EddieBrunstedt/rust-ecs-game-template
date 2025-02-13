use raylib::prelude::*;

use crate::{config::GameConfig, world::World};

use super::{GameScene, SceneTransition};

pub struct SceneManager {
    scene_stack: Vec<GameScene>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scene_stack: vec![],
        }
    }

    pub fn current_scene(&mut self) -> Option<&mut GameScene> {
        self.scene_stack.last_mut()
    }

    pub fn handle_transition(
        &mut self,
        transition: SceneTransition,
        world: &mut World,
        config: &GameConfig,
    ) {
        match transition {
            SceneTransition::Replace(new_scene) => {
                if let Some(mut current_scene) = self.scene_stack.pop() {
                    current_scene.exit(world);
                }
                self.scene_stack.push(new_scene);
                self.scene_stack.last_mut().unwrap().enter(world, config);
            }
            SceneTransition::Push(new_scene) => {
                self.scene_stack.push(new_scene);
                self.scene_stack.last_mut().unwrap().enter(world, config);
            }
            SceneTransition::Pop => {
                if let Some(mut current_scene) = self.scene_stack.pop() {
                    current_scene.exit(world);
                }
            }
        }
    }

    pub fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        let first_non_transparent = self
            .scene_stack
            .iter()
            .rev()
            .position(|scene| !scene.is_transparent())
            .map(|pos| self.scene_stack.len() - 1 - pos);

        match first_non_transparent {
            Some(base_index) => {
                for scene in self.scene_stack[base_index..].iter_mut() {
                    scene.render(world, draw_handle, config);
                }
            }
            None => {
                for scene in self.scene_stack.iter_mut() {
                    scene.render(world, draw_handle, config);
                }
            }
        }
    }
}

use raylib::prelude::*;

use crate::{config::GameConfig, world::World};

use super::{GameScene, SceneTransition};

/// Manages a stack of game scenes, handling scene transitions and rendering.
///
/// The `SceneManager` maintains a stack-based approach to scene management, where scenes can be
/// pushed, popped, or replaced. This allows for layered UI states, menu overlays, and scene
/// transitions while maintaining proper scene lifecycle management.
pub struct SceneManager {
    /// Stack of active game scenes, with the top-most scene being the currently active one.
    scene_stack: Vec<GameScene>,
}

impl SceneManager {
    /// Creates a new empty scene manager.
    pub fn new() -> Self {
        Self {
            scene_stack: vec![],
        }
    }

    /// Creates a new scene manager initialized with a single scene.
    ///
    /// # Arguments
    /// * `scene` - The initial scene to start with
    pub fn with_scene(scene: GameScene) -> Self {
        let mut manager = Self::new();
        manager.scene_stack.push(scene);
        manager
    }

    /// Returns a mutable reference to the current (top-most) scene, if any exists.
    pub fn current_scene(&mut self) -> Option<&mut GameScene> {
        self.scene_stack.last_mut()
    }

    /// Handles scene transitions by managing the scene stack according to the transition type.
    ///
    /// # Arguments
    /// * `transition` - The type of transition to perform (Replace, Push, or Pop)
    /// * `world` - The game world state
    /// * `config` - The game configuration
    ///
    /// # Transitions
    /// * `Replace` - Removes current scene and adds new scene
    /// * `Push` - Adds new scene on top of current scene
    /// * `Pop` - Removes current scene
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

    /// Renders scenes in the stack from back to front.
    /// If a non-transparent scene exists, renders from that scene upward.
    /// Otherwise, renders the entire stack.
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

//! Core game loop and state management.

use crate::{
    config::GameConfig,
    world::{components::*, systems::*, EntityId, World},
};
use raylib::prelude::*;

/// Main game state container and controller.
pub struct Game {
    raylib_handle: RaylibHandle,
    state: GameState,
    raylib_thread: RaylibThread,
    world: World,
}

/// Represents the different states the game can be in.
#[allow(dead_code)]
pub enum GameState {
    GameOver,
    Loading,
    MainMenu,
    Paused,
    Playing,
}

/// Builder pattern implementation for game initialization.
/// Handles the setup of core game systems and entities.
pub struct GameBuilder<'a> {
    world: World,
    config: &'a GameConfig,
}

impl<'a> GameBuilder<'a> {
    pub fn new(config: &'a GameConfig) -> Self {
        Self {
            world: World::new(),
            config,
        }
    }

    fn setup_player(&mut self) -> EntityId {
        let player = self.world.create_entity();

        self.world
            .add_position(player, PositionComponent(Vector2::new(0.0, 0.0)));

        self.world.add_player(
            player,
            PlayerComponent {
                is_local: true,
                speed: self.config.player_speed,
            },
        );

        self.world.add_controls(
            player,
            ControlComponent {
                movement: Vector2::zero(),
            },
        );

        self.world.add_visual(
            player,
            VisualComponent {
                color: Color::RED,
                size: self.config.player_size,
            },
        );

        player
    }

    fn setup_camera(&mut self, following_entity: EntityId) {
        let camera = self.world.create_entity();

        self.world
            .add_position(camera, PositionComponent(Vector2::zero()));

        self.world.add_camera(
            camera,
            CameraComponent {
                offset: Vector2::new(
                    self.config.window_width as f32 / 2.,
                    self.config.window_height as f32 / 2.,
                ),
                rotation: 0.0,
                zoom: self.config.camera_zoom,
                following_entity: Some(following_entity),
            },
        );
    }

    fn setup_background(&mut self) {
        for _ in 0..self.config.background_dots_count {
            let dot = self.world.create_entity();
            self.world.add_position(
                dot,
                PositionComponent(Vector2::new(
                    rand::random::<f32>() * 1000.0 - 500.0,
                    rand::random::<f32>() * 1000.0 - 500.0,
                )),
            );
            self.world.add_visual(
                dot,
                VisualComponent {
                    color: Color::GRAY,
                    size: self.config.background_dot_size,
                },
            );
        }
    }

    pub fn build(mut self) -> Game {
        let player = self.setup_player();
        self.setup_camera(player);
        self.setup_background();

        let (raylib_handle, thread) = raylib::init()
            .size(self.config.window_width, self.config.window_height)
            .title(self.config.window_title)
            .build();

        Game {
            raylib_handle,
            raylib_thread: thread,
            state: GameState::Playing,
            world: self.world,
        }
    }
}

impl Game {
    pub fn run(&mut self) {
        while !self.raylib_handle.window_should_close() {
            self.update_current_state();
        }
    }

    fn update_current_state(&mut self) {
        match self.state {
            GameState::Playing => self.handle_playing_state(),
            GameState::GameOver => todo!(),
            GameState::Loading => todo!(),
            GameState::MainMenu => todo!(),
            GameState::Paused => todo!(),
        }
    }

    fn handle_playing_state(&mut self) {
        HandleInputSystem::run(&mut self.world, &self.raylib_handle);
        UpdatePlayerPositionsSystem::run(&mut self.world, &self.raylib_handle);
        CollisionDetectionSystem::run(&mut self.world, &self.raylib_handle);
        CameraFollowSystem::run(&mut self.world, &self.raylib_handle);
        RenderVisualsSystem::run(
            &mut self.world,
            &mut self.raylib_handle.begin_drawing(&self.raylib_thread),
        );
    }
}

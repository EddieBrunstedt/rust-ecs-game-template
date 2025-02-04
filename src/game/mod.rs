//! Core game loop and state management.

use crate::{config::GameConfig, world::*};
use raylib::prelude::*;

/// Main game state container and controller.
pub struct Game {
    config: GameConfig,
    raylib_handle: RaylibHandle,
    raylib_thread: RaylibThread,
    state: GameState,
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

    fn setup_resources(&mut self) {
        self.world
            .resources
            .insert_game_time(GameTime { total: 0.0 });
    }

    fn setup_player(&mut self) -> Entity {
        let player = self
            .world
            .build_entity()
            .with(PlayerComponent {
                speed: self.config.player_speed,
            })
            .with(PositionComponent {
                coords: Vector2::zero(),
            })
            .with(ControlsComponent {
                movement_direction: Vector2::zero(),
            })
            .with(VisualComponent {
                color: self.config.player_color,
                size: self.config.player_size,
            })
            .id();

        player
    }

    fn setup_camera(&mut self, following_entity: Entity) {
        self.world
            .build_entity()
            .with(CameraComponent {
                offset: Vector2::new(
                    self.config.window_width as f32 / 2.,
                    self.config.window_height as f32 / 2.,
                ),
                rotation: self.config.camera_rotation,
                zoom: self.config.camera_zoom,
                following_entity: Some(following_entity),
            })
            .with(PositionComponent {
                coords: Vector2::zero(),
            });
    }

    fn setup_background(&mut self) {
        let points = self.config.background_dots_count;

        for i in 0..points {
            let angle = (i as f32 / points as f32)
                * self.config.background_spiral_spins
                * std::f32::consts::TAU;
            let radius = self.config.background_spiral_min_radius
                + (self.config.background_spiral_radius_step * angle / std::f32::consts::TAU);

            let x = angle.cos() * radius;
            let y = angle.sin() * radius;

            self.world
                .build_entity()
                .with(PositionComponent {
                    coords: Vector2::new(x, y),
                })
                .with(VisualComponent {
                    color: Color::GRAY,
                    size: self.config.background_dot_size,
                });
        }
    }

    fn setup_score(&mut self) {
        self.world.build_entity().with(ScoreComponent { value: 0 });
    }

    pub fn build(mut self) -> Game {
        let player = self.setup_player();
        self.setup_camera(player);
        self.setup_resources();
        self.setup_background();
        self.setup_score();

        let (raylib_handle, raylib_thread) = raylib::init()
            .size(self.config.window_width, self.config.window_height)
            .title(self.config.window_title)
            .build();

        Game {
            config: self.config,
            raylib_handle,
            raylib_thread,
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
            GameState::MainMenu => todo!(),
            GameState::Paused => todo!(),
        }
    }

    fn handle_playing_state(&mut self) {
        // Update Systems:
        UpdateTimeSystem::run(&mut self.world, &self.raylib_handle, &self.config);
        HandleInputSystem::run(&mut self.world, &self.raylib_handle, &self.config);
        UpdatePlayerPositionsSystem::run(&mut self.world, &self.raylib_handle, &self.config);
        CollisionDetectionSystem::run(&mut self.world, &self.raylib_handle, &self.config);
        CameraFollowSystem::run(&mut self.world, &self.raylib_handle, &self.config);

        // Render Systems:
        let mut draw_handle = self.raylib_handle.begin_drawing(&self.raylib_thread);
        ClearScreenSystem::run(&mut self.world, &mut draw_handle, &self.config);
        RenderVisualsSystem::run(&mut self.world, &mut draw_handle, &self.config);
        RenderScoreSystem::run(&mut self.world, &mut draw_handle, &self.config);
        RenderGameTimeSystem::run(&mut self.world, &mut draw_handle, &self.config);
        RenderFpsSystem::run(&mut self.world, &mut draw_handle, &self.config);
    }
}

/// Represents the different states the game can be in.
/// Controls which update and render systems are active.
#[allow(dead_code)]
enum GameState {
    GameOver,
    MainMenu,
    Paused,
    Playing,
}

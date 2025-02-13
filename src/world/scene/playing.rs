use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{system::*, *},
};

use super::{Scene, SceneTransition};

pub struct PlayingScene;

impl Scene for PlayingScene {
    fn enter(&mut self, world: &mut World, config: &GameConfig) {
        let player = setup_player(world, config);
        setup_camera(world, config, player);
        setup_resources(world);
        setup_background(world, config);
        setup_score(world);
        setup_ui_state(world, config);
    }

    fn exit(&mut self, _world: &mut World) {}

    fn is_transparent(&self) -> bool {
        false
    }

    fn update(
        &mut self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
        let systems_to_run = [
            GameUpdateSystem::HandlePause(HandlePauseSystem),
            GameUpdateSystem::UpdateTime(UpdateTimeSystem),
            GameUpdateSystem::HandleInput(HandleInputSystem),
            GameUpdateSystem::UpdatePlayerPositions(UpdatePlayerPositionsSystem),
            GameUpdateSystem::CollisionDetection(CollisionDetectionSystem),
            GameUpdateSystem::CameraFollow(CameraFollowSystem),
        ];

        for system in systems_to_run {
            if let Some(transition) = system.run(world, raylib_handle, config) {
                return Some(transition);
            }
        }

        None
    }

    fn render(
        &mut self,
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        RenderVisualsSystem::run(world, draw_handle, config);
        RenderScoreSystem::run(world, draw_handle, config);
        RenderGameTimeSystem::run(world, draw_handle, config);
        RenderFpsSystem::run(world, draw_handle, config);
        RenderPressPForPausSystem::run(world, draw_handle, config);
    }
}

fn setup_resources(world: &mut World) {
    world
        .resources
        .insert_game_time(GameTimeResource { total: 0.0 });
}

fn setup_player(world: &mut World, config: &GameConfig) -> Entity {
    let player = world
        .build_entity()
        .with(PlayerComponent {
            speed: config.player_speed,
        })
        .with(PositionComponent {
            coords: Vector2::zero(),
        })
        .with(ControlsComponent {
            movement_direction: Vector2::zero(),
        })
        .with(VisualComponent {
            color: config.player_color,
            size: config.player_size,
        })
        .id();

    player
}

fn setup_camera(world: &mut World, config: &GameConfig, entity_to_follow: Entity) {
    world
        .build_entity()
        .with(CameraComponent {
            offset: Vector2::new(
                config.window_width as f32 / 2.,
                config.window_height as f32 / 2.,
            ),
            rotation: config.camera_rotation,
            zoom: config.camera_zoom,
            following_entity: Some(entity_to_follow),
        })
        .with(PositionComponent {
            coords: Vector2::zero(),
        });
}

fn setup_background(world: &mut World, config: &GameConfig) {
    let points = config.background_dots_count;

    for i in 0..points {
        let angle =
            (i as f32 / points as f32) * config.background_spiral_spins * std::f32::consts::TAU;
        let radius = config.background_spiral_min_radius
            + (config.background_spiral_radius_step * angle / std::f32::consts::TAU);

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        world
            .build_entity()
            .with(PositionComponent {
                coords: Vector2::new(x, y),
            })
            .with(VisualComponent {
                color: Color::GRAY,
                size: config.background_dot_size,
            });
    }
}

fn setup_score(world: &mut World) {
    world.build_entity().with(ScoreComponent { value: 0 });
}

fn setup_ui_state(world: &mut World, config: &GameConfig) {
    world.build_entity().with(UiVisibilityComponent {
        fps_counter: config.fps_initial_visibility,
        press_p_for_paus: true,
    });
}

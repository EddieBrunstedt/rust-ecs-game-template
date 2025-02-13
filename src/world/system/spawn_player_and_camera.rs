use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{component::*, SceneTransition, World},
};

use super::UpdateSystem;

pub struct SpawnPlayerAndCameraSystem;

impl UpdateSystem for SpawnPlayerAndCameraSystem {
    fn run(
        &self,
        world: &mut World,
        _raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
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

        world
            .build_entity()
            .with(CameraComponent {
                offset: Vector2::new(
                    config.window_width as f32 / 2.,
                    config.window_height as f32 / 2.,
                ),
                rotation: config.camera_rotation,
                zoom: config.camera_zoom,
                following_entity: Some(player),
            })
            .with(PositionComponent {
                coords: Vector2::zero(),
            });

        None
    }
}

use crate::{config::GameConfig, player::Player};
use raylib::prelude::*;

pub struct World {
    pub config: GameConfig,
    pub player: Player,
    pub background_dots: Vec<Vector2>,
    pub camera: Camera2D,
}

impl World {
    pub fn new(config: &GameConfig) -> Self {
        let player = Player {
            position: Vector2::new(0.0, 0.0),
            speed: config.player_speed,
        };

        let background_dots: Vec<Vector2> = (0..config.background_dots_count)
            .map(|_| {
                Vector2::new(
                    rand::random::<f32>() * 1000.0 - 500.0,
                    rand::random::<f32>() * 1000.0 - 500.0,
                )
            })
            .collect();

        let camera = Camera2D {
            target: player.position,
            offset: Vector2::new(
                config.window_width as f32 / 2.,
                config.window_height as f32 / 2.,
            ),
            rotation: 0.0,
            zoom: config.camera_zoom,
        };

        Self {
            config: config.clone(),
            player,
            background_dots,
            camera,
        }
    }
}

use raylib::prelude::*;

use crate::{
    config::GameConfig,
    world::{component::*, SceneTransition, World},
};

use super::UpdateSystem;

pub struct SpawnSpiralDots;

impl UpdateSystem for SpawnSpiralDots {
    fn run(
        &self,
        world: &mut World,
        _raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
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

        None
    }
}

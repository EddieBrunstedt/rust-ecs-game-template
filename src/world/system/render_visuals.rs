use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Renders all visible entities to the screen.
pub struct RenderVisualsSystem;

impl RenderSystem for RenderVisualsSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, _config: &GameConfig) {
        let (camera, position) = world
            .entity_ids
            .iter()
            .find_map(|&id| {
                Some((
                    world.components.camera(id)?,
                    world.components.expect_position(id),
                ))
            })
            .expect("No camera entity found in world");

        let mut draw = draw_handle.begin_mode2D(Camera2D {
            target: position.coords,
            offset: camera.offset,
            rotation: camera.rotation,
            zoom: camera.zoom,
        });

        world
            .entity_ids
            .iter()
            .filter_map(|&id| Some((world.components.position(id)?, world.components.visual(id)?)))
            .for_each(|(position, visual)| {
                draw.draw_circle(
                    position.coords.x as i32,
                    position.coords.y as i32,
                    visual.size,
                    visual.color,
                )
            });
    }
}

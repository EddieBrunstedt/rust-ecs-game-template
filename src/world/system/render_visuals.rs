use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Renders all camera-visible entities to the screen.
// TODO: Rename to convey this is camera stuff (and refactor)
pub struct RenderVisualsSystem;

impl RenderSystem for RenderVisualsSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, _config: &GameConfig) {
        let (camera, position) = world
            .iter_entities()
            .find_map(|id| {
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
            .iter_entities()
            .filter_map(|id| Some((world.components.position(id)?, world.components.visual(id)?)))
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

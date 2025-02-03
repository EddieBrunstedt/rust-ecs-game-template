use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Clears the entire screen.
/// Commonly used at the beginning of each rendering cycle.
pub struct ClearScreenSystem;

impl RenderSystem for ClearScreenSystem {
    fn run(_world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        draw_handle.clear_background(config.background_color);
    }
}

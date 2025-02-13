use raylib::prelude::*;

use crate::{config::GameConfig, world::World};

use super::RenderSystem;

/// Clears the entire screen.
/// Commonly used at the beginning of each rendering cycle.
pub struct ClearScreenSystem;

impl RenderSystem for ClearScreenSystem {
    fn run(_world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        draw_handle.clear_background(config.background_color);
    }
}

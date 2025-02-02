use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Clears screen
pub struct ClearScreenSystem;

impl RenderSystem for ClearScreenSystem {
    fn run(_world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        draw_handle.clear_background(config.background_color);
    }
}

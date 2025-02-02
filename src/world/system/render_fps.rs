use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Renders the current FPS in the top right corner
pub struct RenderFpsSystem;

impl RenderSystem for RenderFpsSystem {
    fn run(_world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        let fps = draw_handle.get_fps();
        let text = format!("FPS: {}", fps);
        let text_width = draw_handle.measure_text(&text, config.hud_font_size);

        draw_handle.draw_text(
            &text,
            config.window_width - text_width - config.fps_padding_right,
            config.fps_padding_top,
            config.hud_font_size,
            config.hud_font_color,
        );
    }
}

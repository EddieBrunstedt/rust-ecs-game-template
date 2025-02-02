use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Renders the game time on screen
pub struct RenderGameTimeSystem;

impl RenderSystem for RenderGameTimeSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        let game_time = world.resources.expect_game_time();

        draw_handle.draw_text(
            &format!("Time: {:.1}", game_time.total),
            config.score_text_x,
            config.score_text_y + config.hud_font_size + 5,
            config.hud_font_size,
            config.hud_font_color,
        );
    }
}

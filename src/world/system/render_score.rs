use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Renders the score on screen
pub struct RenderScoreSystem;

impl RenderSystem for RenderScoreSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        if let Some(score) = world
            .entity_ids
            .iter()
            .find_map(|e| world.components.score(*e))
        {
            draw_handle.draw_text(
                &format!("Score: {}", score.value),
                config.score_text_x,
                config.score_text_y,
                config.score_font_size,
                config.score_text_color,
            );
        }
    }
}

use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Renders a text saying "Press P to pause the game"
pub struct RenderPressPForPausSystem;

impl RenderSystem for RenderPressPForPausSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        let ui_entity = world
            .iter_entities()
            .find(|e| world.components.ui_visibility(*e).is_some())
            .expect("UI visibility state is required");

        let ui_visibility = world.components.ui_visibility_mut(ui_entity);

        let Some(ui_visibility) = ui_visibility else {
            return;
        };

        if !ui_visibility.press_p_for_paus {
            return;
        };

        let text = "Press P to pause the game";
        let text_width = draw_handle.measure_text(text, config.hud_font_size);

        let x = draw_handle.get_screen_width() - text_width - 10;
        let y = draw_handle.get_screen_height() - config.hud_font_size - 10;

        draw_handle.draw_text(text, x, y, config.hud_font_size, config.hud_font_color);
    }
}

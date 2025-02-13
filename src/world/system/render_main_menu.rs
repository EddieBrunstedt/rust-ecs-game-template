use super::RenderSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

/// Renders the paus menu on screen.
pub struct RenderMainMenu;

impl RenderSystem for RenderMainMenu {
    fn run(_world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig) {
        let title = "MAIN MENU";
        let title_size = 40;
        let title_width = draw_handle.measure_text(title, title_size);

        draw_handle.draw_text(
            title,
            config.window_width / 2 - title_width / 2,
            config.window_height / 3,
            title_size,
            Color::WHITE,
        );

        let options_text = "Press ENTER to play the game";
        let options_size = 20;
        let options_width = draw_handle.measure_text(options_text, options_size);

        draw_handle.draw_text(
            options_text,
            config.window_width / 2 - options_width / 2,
            config.window_height / 2,
            options_size,
            Color::WHITE,
        );
    }
}

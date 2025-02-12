use super::{Scene, SceneTransition};
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

#[derive(Debug)]
pub struct PauseScene;

impl Scene for PauseScene {
    fn enter(&mut self, _world: &mut World, _config: &GameConfig) {}

    fn exit(&mut self, _world: &mut World) {}

    fn is_transparent(&self) -> bool {
        true
    }

    fn update(
        &mut self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        _config: &GameConfig,
    ) -> Option<SceneTransition> {
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            Some(SceneTransition::Pop)
        } else if raylib_handle.is_key_pressed(KeyboardKey::KEY_F) {
            let entities: Vec<_> = world.iter_entities().collect();

            for entity in entities {
                if let Some(ui_visibility) = world.components.ui_visibility_mut(entity) {
                    ui_visibility.fps_counter = !ui_visibility.fps_counter;
                    break;
                }
            }
            None
        } else {
            None
        }
    }

    fn render(
        &mut self,
        _world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        // Draw semi-transparent overlay
        draw_handle.draw_rectangle(
            (config.window_width / 2) / 2,
            (config.window_height / 2) / 2,
            config.window_width / 2,
            config.window_height / 2,
            Color::new(30, 30, 30, 100),
        );

        // Draw menu title
        let title = "PAUSE MENU";
        let title_size = 40;
        let title_width = draw_handle.measure_text(title, title_size);
        draw_handle.draw_text(
            title,
            config.window_width / 2 - title_width / 2,
            config.window_height / 3,
            title_size,
            Color::WHITE,
        );

        // Draw options
        let options_text = "Press F to toggle FPS\n\nPress ESC to resume";
        let options_size = 20;
        let options_width =
            draw_handle.measure_text(options_text.split('\n').next().unwrap(), options_size);
        draw_handle.draw_text(
            options_text,
            config.window_width / 2 - options_width / 2,
            config.window_height / 2,
            options_size,
            Color::WHITE,
        );
    }
}

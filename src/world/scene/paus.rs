use super::{Scene, SceneTransition};
use crate::{
    config::GameConfig,
    world::{system::*, *},
};
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
            let ui_entity = world
                .iter_entities()
                .find(|e| world.components.ui_visibility(*e).is_some())
                .expect("UI visibility state is required");

            world
                .components
                .expect_ui_visibility_mut(ui_entity)
                .press_p_for_paus = true;

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
        world: &mut World,
        draw_handle: &mut RaylibDrawHandle,
        config: &GameConfig,
    ) {
        RenderPausMenu::run(world, draw_handle, config);
    }
}

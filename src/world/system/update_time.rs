use super::UpdateSystem;
use crate::{config::GameConfig, world::World};
use raylib::prelude::*;

pub struct UpdateTimeSystem;

impl UpdateSystem for UpdateTimeSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle, _config: &GameConfig) {
        if let Some(game_time) = world.resources.game_time_mut() {
            game_time.delta = raylib_handle.get_frame_time();
            game_time.total += game_time.delta;
        }
    }
}

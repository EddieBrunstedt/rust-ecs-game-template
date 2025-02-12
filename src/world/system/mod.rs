mod camera_follow;
mod clear_screen;
mod collosion_detection;
mod handle_input;
mod render_fps;
mod render_game_time;
mod render_paus_menu;
mod render_score;
mod render_visuals;
mod update_player_positions;
mod update_time;

use super::World;
use crate::config::GameConfig;
use raylib::prelude::*;

pub use camera_follow::CameraFollowSystem;
pub use clear_screen::ClearScreenSystem;
pub use collosion_detection::CollisionDetectionSystem;
pub use handle_input::HandleInputSystem;
pub use render_fps::RenderFpsSystem;
pub use render_game_time::RenderGameTimeSystem;
pub use render_paus_menu::RenderPausMenu;
pub use render_score::RenderScoreSystem;
pub use render_visuals::RenderVisualsSystem;
pub use update_player_positions::UpdatePlayerPositionsSystem;
pub use update_time::UpdateTimeSystem;

/// Defines behavior for systems that render to the screen.
/// Render systems are called each frame, commonly after update systems complete.
pub trait RenderSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle, config: &GameConfig);
}

/// Defines behavior for systems that update game state.
/// Update systems are called each frame, commonly before rendering occurs.
pub trait UpdateSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig);
}

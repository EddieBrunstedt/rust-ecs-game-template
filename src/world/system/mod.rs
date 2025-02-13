mod camera_follow;
mod clear_screen;
mod collosion_detection;
mod handle_input;
mod handle_pause_system;
mod render_fps;
mod render_game_time;
mod render_main_menu;
mod render_paus_menu;
mod render_press_p_for_paus;
mod render_score;
mod render_visuals;
mod update_player_positions;
mod update_time;

use super::{SceneTransition, World};
use crate::config::GameConfig;
use raylib::prelude::*;

pub use camera_follow::CameraFollowSystem;
pub use clear_screen::ClearScreenSystem;
pub use collosion_detection::CollisionDetectionSystem;
pub use handle_input::HandleInputSystem;
pub use handle_pause_system::HandlePauseSystem;
pub use render_fps::RenderFpsSystem;
pub use render_game_time::RenderGameTimeSystem;
pub use render_main_menu::RenderMainMenu;
pub use render_paus_menu::RenderPausMenu;
pub use render_press_p_for_paus::RenderPressPForPausSystem;
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
    fn run(&self, world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig);
}

pub trait TransitionSystem: UpdateSystem {
    fn check_transition(world: &mut World, raylib_handle: &RaylibHandle)
        -> Option<SceneTransition>;
}

pub enum GameSystem {
    HandlePause(HandlePauseSystem),
    UpdateTime(UpdateTimeSystem),
    HandleInput(HandleInputSystem),
    UpdatePlayerPositions(UpdatePlayerPositionsSystem),
    CollisionDetection(CollisionDetectionSystem),
    CameraFollow(CameraFollowSystem),
}

impl UpdateSystem for GameSystem {
    fn run(&self, world: &mut World, raylib_handle: &RaylibHandle, config: &GameConfig) {
        match self {
            GameSystem::HandlePause(sys) => sys.run(world, raylib_handle, config),
            GameSystem::UpdateTime(sys) => sys.run(world, raylib_handle, config),
            GameSystem::HandleInput(sys) => sys.run(world, raylib_handle, config),
            GameSystem::UpdatePlayerPositions(sys) => sys.run(world, raylib_handle, config),
            GameSystem::CollisionDetection(sys) => sys.run(world, raylib_handle, config),
            GameSystem::CameraFollow(sys) => sys.run(world, raylib_handle, config),
        }
    }
}

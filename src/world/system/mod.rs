mod camera_follow;
mod clear_screen;
mod collosion_detection;
mod handle_input;
mod handle_pause;
mod handle_press_enter_to_play;
mod handle_press_esc_to_play;
mod handle_press_f_to_show_fps;
mod render_fps;
mod render_game_time;
mod render_main_menu;
mod render_paus_menu;
mod render_press_p_for_paus;
mod render_score;
mod render_visuals;
mod update_player_positions;
mod update_time;

use raylib::prelude::*;

use crate::config::GameConfig;

use super::{SceneTransition, World};

pub use camera_follow::CameraFollowSystem;
pub use clear_screen::ClearScreenSystem;
pub use collosion_detection::CollisionDetectionSystem;
pub use handle_input::HandleInputSystem;
pub use handle_pause::HandlePauseSystem;
pub use handle_press_enter_to_play::HandlePressEnterToPlaySystem;
pub use handle_press_esc_to_play::HandlePressEscToPlaySystem;
pub use handle_press_f_to_show_fps::HandlePressFToShowFpsSystem;
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
    /// Returns Some(SceneTransition) if a transition should occur, None otherwise
    fn run(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition>;
}

pub enum GameUpdateSystem {
    CameraFollow(CameraFollowSystem),
    CollisionDetection(CollisionDetectionSystem),
    HandleInput(HandleInputSystem),
    HandlePause(HandlePauseSystem),
    HandlePressEnterToPlay(HandlePressEnterToPlaySystem),
    HandlePressEscToPlay(HandlePressEscToPlaySystem),
    HandlePressFToShowFps(HandlePressFToShowFpsSystem),
    UpdatePlayerPositions(UpdatePlayerPositionsSystem),
    UpdateTime(UpdateTimeSystem),
}

impl UpdateSystem for GameUpdateSystem {
    fn run(
        &self,
        world: &mut World,
        raylib_handle: &RaylibHandle,
        config: &GameConfig,
    ) -> Option<SceneTransition> {
        match self {
            GameUpdateSystem::CameraFollow(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::CollisionDetection(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandleInput(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePause(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePressEnterToPlay(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePressEscToPlay(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePressFToShowFps(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::UpdatePlayerPositions(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::UpdateTime(sys) => sys.run(world, raylib_handle, config),
        }
    }
}

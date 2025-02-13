mod camera_follow;
mod clear_screen;
mod collosion_detection;
mod create_game_time_resource;
mod create_score;
mod create_ui_visibility;
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
mod spawn_player_and_camera;
mod spawn_spiral_dots;
mod update_player_positions;
mod update_time;

use raylib::prelude::*;

use crate::config::GameConfig;

use super::{SceneTransition, World};

pub use self::{
    camera_follow::CameraFollowSystem, clear_screen::ClearScreenSystem,
    collosion_detection::CollisionDetectionSystem,
    create_game_time_resource::CreateGameTimeResourceSystem, create_score::CreateScore,
    create_ui_visibility::CreateUiVisibility, handle_input::HandleInputSystem,
    handle_pause::HandlePauseSystem, handle_press_enter_to_play::HandlePressEnterToPlaySystem,
    handle_press_esc_to_play::HandlePressEscToPlaySystem,
    handle_press_f_to_show_fps::HandlePressFToShowFpsSystem, render_fps::RenderFpsSystem,
    render_game_time::RenderGameTimeSystem, render_main_menu::RenderMainMenu,
    render_paus_menu::RenderPausMenu, render_press_p_for_paus::RenderPressPForPausSystem,
    render_score::RenderScoreSystem, render_visuals::RenderVisualsSystem,
    spawn_player_and_camera::SpawnPlayerAndCameraSystem, spawn_spiral_dots::SpawnSpiralDots,
    update_player_positions::UpdatePlayerPositionsSystem, update_time::UpdateTimeSystem,
};

pub enum GameUpdateSystem {
    CameraFollow(CameraFollowSystem),
    CollisionDetection(CollisionDetectionSystem),
    CreateGameTimeResourceSystem(CreateGameTimeResourceSystem),
    CreateScore(CreateScore),
    HandleInput(HandleInputSystem),
    HandlePause(HandlePauseSystem),
    CreateUiVisibility(CreateUiVisibility),
    HandlePressEnterToPlay(HandlePressEnterToPlaySystem),
    HandlePressEscToPlay(HandlePressEscToPlaySystem),
    HandlePressFToShowFps(HandlePressFToShowFpsSystem),
    SpawnPlayerAndCamera(SpawnPlayerAndCameraSystem),
    SpawnSpiralDots(SpawnSpiralDots),
    UpdatePlayerPositions(UpdatePlayerPositionsSystem),
    UpdateTime(UpdateTimeSystem),
}

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
            GameUpdateSystem::CreateGameTimeResourceSystem(sys) => {
                sys.run(world, raylib_handle, config)
            }
            GameUpdateSystem::CreateScore(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::CreateUiVisibility(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandleInput(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePause(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePressEnterToPlay(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePressEscToPlay(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::HandlePressFToShowFps(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::SpawnPlayerAndCamera(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::SpawnSpiralDots(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::UpdatePlayerPositions(sys) => sys.run(world, raylib_handle, config),
            GameUpdateSystem::UpdateTime(sys) => sys.run(world, raylib_handle, config),
        }
    }
}

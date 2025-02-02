//! Configuration management for the game engine.

use raylib::prelude::*;

/// Represents the core configuration parameters for the game.
/// All values are static and initialized at startup.
pub struct GameConfig {
    pub window_title: &'static str,
    pub window_height: i32,
    pub window_width: i32,

    pub camera_zoom: f32,

    pub player_size: f32,
    pub player_speed: f32,

    pub background_dot_size: f32,
    pub background_dots_count: usize,
    pub background_spiral_min_radius: f32,
    pub background_spiral_radius_step: f32,
    pub background_spiral_spins: f32,

    pub score_font_size: i32,
    pub score_text_color: Color,
    pub score_text_x: i32,
    pub score_text_y: i32,

    pub points_per_collision: i32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_title: "The Game",
            window_height: 480,
            window_width: 640,

            camera_zoom: 1.0,

            player_size: 8.0,
            player_speed: 0.1,

            background_dot_size: 4.0,
            background_dots_count: 100,
            background_spiral_min_radius: 50.0,
            background_spiral_radius_step: 50.0,
            background_spiral_spins: 4.0,

            score_font_size: 20,
            score_text_color: Color::WHITE,
            score_text_x: 10,
            score_text_y: 10,

            points_per_collision: 1,
        }
    }
}

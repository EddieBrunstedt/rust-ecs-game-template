//! Configuration management for the game engine.

use raylib::prelude::*;

/// Represents the core configuration parameters for the game.
pub struct GameConfig {
    pub window_title: &'static str,
    pub window_height: i32,
    pub window_width: i32,

    pub background_color: Color,

    pub camera_zoom: f32,
    pub camera_rotation: f32,

    /// Size of the player's visual representation in pixels
    pub player_size: f32,
    /// Color of the player's visual representation
    pub player_color: Color,
    /// Player movement speed in units per second
    pub player_speed: f32,

    pub key_move_right: KeyboardKey,
    pub key_move_left: KeyboardKey,
    pub key_move_up: KeyboardKey,
    pub key_move_down: KeyboardKey,
    pub normalize_movement: bool,

    pub background_dot_size: f32,
    pub background_dots_count: usize,
    pub background_spiral_min_radius: f32,
    pub background_spiral_radius_step: f32,
    pub background_spiral_spins: f32,

    pub hud_font_size: i32,
    pub hud_font_color: Color,

    pub score_text_x: i32,
    pub score_text_y: i32,

    pub fps_padding_top: i32,
    pub fps_padding_right: i32,

    pub points_per_collision: i32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_title: "The Game",
            window_height: 480,
            window_width: 640,

            background_color: Color::BLACK,

            camera_zoom: 1.0,
            camera_rotation: 0.0,

            player_size: 8.,
            player_color: Color::RED,
            player_speed: 250.,

            key_move_right: KeyboardKey::KEY_RIGHT,
            key_move_left: KeyboardKey::KEY_LEFT,
            key_move_up: KeyboardKey::KEY_UP,
            key_move_down: KeyboardKey::KEY_DOWN,
            normalize_movement: true,

            background_dot_size: 4.0,
            background_dots_count: 100,
            background_spiral_min_radius: 50.0,
            background_spiral_radius_step: 50.0,
            background_spiral_spins: 4.0,

            hud_font_size: 20,
            hud_font_color: Color::WHITE,

            score_text_x: 10,
            score_text_y: 10,

            fps_padding_top: 10,
            fps_padding_right: 10,

            points_per_collision: 1,
        }
    }
}

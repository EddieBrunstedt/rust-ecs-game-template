use std::sync::LazyLock;

pub struct GameConfig {
    pub window_width: i32,
    pub window_height: i32,
    pub window_title: &'static str,

    pub player_speed: f32,
    pub player_size: f32,

    pub background_dots_count: usize,
    pub background_dot_size: f32,

    pub camera_zoom: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 640,
            window_height: 480,
            window_title: "Le Game",

            player_speed: 0.1,
            player_size: 8.0,

            background_dots_count: 50,
            background_dot_size: 4.0,

            camera_zoom: 1.0,
        }
    }
}

pub static DEFAULT_CONFIG: LazyLock<GameConfig> = LazyLock::new(GameConfig::default);

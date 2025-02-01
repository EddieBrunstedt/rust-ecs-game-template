mod config;
mod player;
mod world;
use config::DEFAULT_CONFIG;
use raylib::prelude::*;
use world::World;

fn main() {
    let config = &DEFAULT_CONFIG;
    let mut world = World::new(config);

    let (mut rl, thread) = raylib::init()
        .size(world.config.window_width, world.config.window_height)
        .title(world.config.window_title)
        .build();

    while !rl.window_should_close() {
        let mut movement = Vector2::zero();

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            movement.x += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            movement.x -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            movement.y += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            movement.y -= 1.0;
        }

        world.player.position += movement * world.player.speed;

        // Update camera to follow player
        world.camera.target = world.player.position;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Begin 2D camera mode
        let mut camera_draw = d.begin_mode2D(world.camera);

        // Draw background dots
        for dot_pos in &world.background_dots {
            camera_draw.draw_circle(
                dot_pos.x as i32,
                dot_pos.y as i32,
                world.config.background_dot_size,
                Color::GRAY,
            );
        }

        // Draw player dot (will appear centered due to camera)
        camera_draw.draw_circle(
            world.player.position.x as i32,
            world.player.position.y as i32,
            world.config.player_size,
            Color::RED,
        );
    }
}

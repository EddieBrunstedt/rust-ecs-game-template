mod config;
mod world;
use config::DEFAULT_CONFIG;
use raylib::prelude::*;
use world::{components::*, systems::*, World};

fn main() {
    let config = &DEFAULT_CONFIG;
    let mut world = World::new();

    let player_entity = world.create_entity();

    world.add_position(player_entity, Position(Vector2::new(0.0, 0.0)));

    world.add_player(
        player_entity,
        Player {
            id: 0,
            is_local: true,
            speed: config.player_speed,
        },
    );

    world.add_input(
        player_entity,
        Input {
            movement: Vector2::zero(),
            player_id: 0,
        },
    );

    world.add_renderable(
        player_entity,
        Renderable {
            color: Color::RED,
            size: config.player_size,
        },
    );

    // Create background dots
    for _ in 0..config.background_dots_count {
        let dot = world.create_entity();
        world.add_position(
            dot,
            Position(Vector2::new(
                rand::random::<f32>() * 1000.0 - 500.0,
                rand::random::<f32>() * 1000.0 - 500.0,
            )),
        );
        world.add_renderable(
            dot,
            Renderable {
                color: Color::GRAY,
                size: config.background_dot_size,
            },
        );
    }

    let (mut rl_handle, thread) = raylib::init()
        .size(config.window_width, config.window_height)
        .title(config.window_title)
        .build();

    let mut camera = Camera2D {
        target: Vector2::zero(),
        offset: Vector2::new(
            config.window_width as f32 / 2.,
            config.window_height as f32 / 2.,
        ),
        rotation: 0.0,
        zoom: config.camera_zoom,
    };

    while !rl_handle.window_should_close() {
        input_system(&mut world, &rl_handle);
        player_movement_system(&mut world);

        // Center camera to player
        if let Some(pos) = world.get_position(player_entity) {
            camera.target = pos.0;
        }

        let mut draw_handle = rl_handle.begin_drawing(&thread);

        draw_handle.clear_background(Color::BLACK);

        render_system(&world, &mut draw_handle, &camera);
    }
}

mod config;
mod world;
use config::DEFAULT_CONFIG;
use raylib::prelude::*;
use world::{components::*, systems::*, World};

fn main() {
    let config = &DEFAULT_CONFIG;

    let mut world = World::new();

    let player_entity = world.create_entity();
    let camera_entity = world.create_entity();

    world.add_position(player_entity, Position(Vector2::new(0.0, 0.0)));

    world.add_player(
        player_entity,
        PlayerComponent {
            id: 0,
            is_local: true,
            speed: config.player_speed,
        },
    );

    world.add_controls(
        player_entity,
        ControlComponent {
            movement: Vector2::zero(),
        },
    );

    world.add_visual(
        player_entity,
        VisualComponent {
            color: Color::RED,
            size: config.player_size,
        },
    );

    world.add_position(camera_entity, Position(Vector2::zero()));

    world.add_camera(
        camera_entity,
        CameraComponent {
            offset: Vector2::new(
                config.window_width as f32 / 2.,
                config.window_height as f32 / 2.,
            ),
            rotation: 0.0,
            zoom: config.camera_zoom,
            following_entity: Some(player_entity),
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
        world.add_visual(
            dot,
            VisualComponent {
                color: Color::GRAY,
                size: config.background_dot_size,
            },
        );
    }

    let (mut raylib_handle, thread) = raylib::init()
        .size(config.window_width, config.window_height)
        .title(config.window_title)
        .build();

    while !raylib_handle.window_should_close() {
        HandleInputSystem::run(&mut world, &raylib_handle);
        UpdatePlayerPositionsSystem::run(&mut world, &raylib_handle);
        CameraFollowSystem::run(&mut world, &raylib_handle);
        RenderVisualsSystem::run(&mut world, &mut raylib_handle.begin_drawing(&thread));
    }
}

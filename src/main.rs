mod config;
mod ecs;
use config::DEFAULT_CONFIG;
use ecs::{components::*, systems::*, World as EcsWorld};
use raylib::prelude::*;

fn main() {
    let config = &DEFAULT_CONFIG;
    let mut ecs_world = EcsWorld::new();

    // Register components
    ecs_world.register_component::<Position>();
    ecs_world.register_component::<Renderable>();
    ecs_world.register_component::<Player>();

    // Create player entity
    let player = ecs_world.create_entity();
    ecs_world.add_component(player, Position(Vector2::new(0.0, 0.0)));
    ecs_world.add_component(
        player,
        Player {
            speed: config.player_speed,
        },
    );
    ecs_world.add_component(
        player,
        Renderable {
            color: Color::RED,
            size: config.player_size,
        },
    );

    // Create background dots
    for _ in 0..config.background_dots_count {
        let dot = ecs_world.create_entity();
        ecs_world.add_component(
            dot,
            Position(Vector2::new(
                rand::random::<f32>() * 1000.0 - 500.0,
                rand::random::<f32>() * 1000.0 - 500.0,
            )),
        );
        ecs_world.add_component(
            dot,
            Renderable {
                color: Color::GRAY,
                size: config.background_dot_size,
            },
        );
    }

    let (mut rl, thread) = raylib::init()
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

        // Update systems
        movement_system(&mut ecs_world, movement);

        // Update camera to follow player
        if let Some(pos) = ecs_world.get_component::<Position>(player) {
            camera.target = pos.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        render_system(&ecs_world, &mut d, &camera);
    }
}

use super::components::*;
use super::EntityId;
use super::World;
use raylib::prelude::*;

pub fn player_movement_system(world: &mut World) {
    let updates: Vec<(EntityId, Vector2)> = world
        .entities
        .iter()
        .filter_map(|entity| {
            let player = world.get_player(*entity)?;
            let input = world.get_input(*entity)?;
            let position = world.get_position(*entity)?;
            let new_pos = position.0 + (input.movement * player.speed);
            Some((*entity, new_pos))
        })
        .collect();

    for (entity, new_pos) in updates {
        world.add_position(entity, Position(new_pos));
    }
}

pub fn render_system(world: &World, d: &mut RaylibDrawHandle, camera: &Camera2D) {
    let mut camera_draw = d.begin_mode2D(*camera);

    for entity in &world.entities {
        if let (Some(position), Some(renderable)) =
            (world.get_position(*entity), world.get_renderable(*entity))
        {
            camera_draw.draw_circle(
                position.0.x as i32,
                position.0.y as i32,
                renderable.size,
                renderable.color,
            );
        }
    }
}

pub fn input_system(world: &mut World, rl: &RaylibHandle) {
    let input_updates: Vec<(EntityId, Vector2)> = world
        .entities
        .iter()
        .filter_map(|entity| {
            let player = world.get_player(*entity)?;

            // Ensures the entity has an Input component attached
            world.get_input(*entity)?;

            if player.is_local {
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

                Some((*entity, movement))
            } else {
                todo!("Implement movement for network players");
            }
        })
        .collect();

    for (entity, movement) in input_updates {
        world.add_input(
            entity,
            Input {
                movement: movement.normalized(),
                player_id: world
                    .get_player(entity)
                    .unwrap_or_else(|| panic!("Could not find entity '{}' in world.", entity))
                    .id,
            },
        );
    }
}

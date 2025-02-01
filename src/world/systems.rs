use super::components::*;
use super::EntityId;
use super::World;
use raylib::prelude::*;

pub fn update_player_positions(world: &mut World) {
    let updates: Vec<(EntityId, Vector2)> = world
        .entity_ids
        .iter()
        .filter_map(|entity| {
            let player = world.get_player(*entity)?;
            let control = world.get_control(*entity)?;
            let position = world.get_position(*entity)?;
            let new_pos = position.0 + (control.movement * player.speed);
            Some((*entity, new_pos))
        })
        .collect();

    for (entity, new_pos) in updates {
        world.add_position(entity, Position(new_pos));
    }
}

pub fn render_visuals(world: &World, draw_handle: &mut RaylibDrawHandle, camera: &Camera2D) {
    let mut camera_draw = draw_handle.begin_mode2D(*camera);

    for entity in &world.entity_ids {
        if let (Some(position), Some(visual)) =
            (world.get_position(*entity), world.get_visual(*entity))
        {
            camera_draw.draw_circle(
                position.0.x as i32,
                position.0.y as i32,
                visual.size,
                visual.color,
            );
        }
    }
}

pub fn handle_input(world: &mut World, raylib_handle: &RaylibHandle) {
    let control_updates: Vec<(EntityId, Vector2)> = world
        .entity_ids
        .iter()
        .filter_map(|entity| {
            let player = world.get_player(*entity)?;

            // Ensures the entity has an Control component attached
            world.get_control(*entity)?;

            if player.is_local {
                let mut movement = Vector2::zero();
                if raylib_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
                    movement.x += 1.0;
                }
                if raylib_handle.is_key_down(KeyboardKey::KEY_LEFT) {
                    movement.x -= 1.0;
                }
                if raylib_handle.is_key_down(KeyboardKey::KEY_DOWN) {
                    movement.y += 1.0;
                }
                if raylib_handle.is_key_down(KeyboardKey::KEY_UP) {
                    movement.y -= 1.0;
                }

                Some((*entity, movement))
            } else {
                todo!("Implement movement for network players");
            }
        })
        .collect();

    for (entity, movement) in control_updates {
        world.add_controls(
            entity,
            Control {
                movement: movement.normalized(),
                player_id: world
                    .get_player(entity)
                    .unwrap_or_else(|| panic!("Could not find entity '{}' in world.", entity))
                    .id,
            },
        );
    }
}

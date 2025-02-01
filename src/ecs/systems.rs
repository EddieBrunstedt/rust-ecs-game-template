use super::components::*;
use super::EntityId;
use super::World;
use raylib::prelude::*;

pub fn player_movement_system(world: &mut World, movement_input: Vector2) {
    if movement_input == Vector2::zero() {
        return;
    }

    let updates: Vec<(EntityId, Vector2)> = world
        .entities
        .iter()
        .filter_map(|entity| {
            let player = world.get_component::<Player>(*entity)?;
            let position = world.get_component::<Position>(*entity)?;
            let new_pos = position.0 + (movement_input * player.speed);
            Some((*entity, new_pos))
        })
        .collect();

    for (entity, new_pos) in updates {
        world.add_component(entity, Position(new_pos));
    }
}

pub fn render_system(world: &World, d: &mut RaylibDrawHandle, camera: &Camera2D) {
    let mut camera_draw = d.begin_mode2D(*camera);

    for entity in &world.entities {
        if let (Some(position), Some(renderable)) = (
            world.get_component::<Position>(*entity),
            world.get_component::<Renderable>(*entity),
        ) {
            camera_draw.draw_circle(
                position.0.x as i32,
                position.0.y as i32,
                renderable.size,
                renderable.color,
            );
        }
    }
}

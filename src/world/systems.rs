use super::EntityId;
use super::World;
use raylib::prelude::*;

pub trait UpdateSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle);
}
pub trait RenderSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle);
}

pub struct CameraFollowSystem;
pub struct CollisionDetectionSystem;
pub struct HandleInputSystem;
pub struct RenderVisualsSystem;
pub struct UpdatePlayerPositionsSystem;

impl UpdateSystem for CollisionDetectionSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle) {
        let player_data: Vec<(u64, Vector2, f32)> = world
            .entity_ids
            .iter()
            .filter_map(|&entity| {
                if world.is_player(entity) {
                    let position = world.expect_position(entity);
                    let visual = world.get_visual(entity)?;
                    Some((entity, position.0, visual.size))
                } else {
                    None
                }
            })
            .collect();

        let entities_to_remove: Vec<EntityId> = world
            .entity_ids
            .iter()
            .filter_map(|&entity| {
                if world.is_player(entity) {
                    return None;
                }

                let position = world.expect_position(entity);
                let visual = world.get_visual(entity)?;

                // Check collision with any player
                for (_player_entity, player_pos, player_size) in &player_data {
                    let distance = (*player_pos - position.0).length();
                    if distance < (player_size + visual.size) {
                        return Some(entity);
                    }
                }
                None
            })
            .collect();

        for entity in entities_to_remove {
            world.remove_entity(entity);
        }
    }
}

impl UpdateSystem for HandleInputSystem {
    fn run(world: &mut World, raylib_handle: &RaylibHandle) {
        let control_updates: Vec<(EntityId, Vector2)> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let player = world.get_player(*entity)?;
                world.expect_control(*entity);

                if player.is_local {
                    let mut movement = Vector2::zero();
                    if raylib_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
                        movement.x += 1.0
                    }
                    if raylib_handle.is_key_down(KeyboardKey::KEY_LEFT) {
                        movement.x -= 1.0
                    }
                    if raylib_handle.is_key_down(KeyboardKey::KEY_DOWN) {
                        movement.y += 1.0
                    }
                    if raylib_handle.is_key_down(KeyboardKey::KEY_UP) {
                        movement.y -= 1.0
                    }

                    Some((*entity, movement))
                } else {
                    todo!("Implement movement for network players");
                }
            })
            .collect();

        for (entity, movement) in control_updates {
            world.expect_control_mut(entity).movement = movement.normalized();
        }
    }
}

impl UpdateSystem for CameraFollowSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle) {
        let updates: Vec<_> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let camera = world.get_camera(*entity)?;
                let target_entity = camera.following_entity?;
                let target_position = world.expect_position(target_entity);
                Some((*entity, target_position.0))
            })
            .collect();

        for (entity, new_position) in updates {
            world.expect_position_mut(entity).0 = new_position;
        }
    }
}

impl UpdateSystem for UpdatePlayerPositionsSystem {
    fn run(world: &mut World, _raylib_handle: &RaylibHandle) {
        let updates: Vec<(EntityId, Vector2)> = world
            .entity_ids
            .iter()
            .filter_map(|entity| {
                let player = world.get_player(*entity)?;
                let control = world.expect_control(*entity);
                let position = world.expect_position(*entity);
                let new_position = position.0 + (control.movement * player.speed);
                Some((*entity, new_position))
            })
            .collect();

        for (entity, new_position) in updates {
            world.expect_position_mut(entity).0 = new_position;
        }
    }
}

impl RenderSystem for RenderVisualsSystem {
    fn run(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.clear_background(Color::BLACK);

        let camera_data = world
            .entity_ids
            .iter()
            .find_map(|entity| {
                let camera = world.get_camera(*entity)?;
                let position = world.expect_position(*entity);
                Some((camera, position))
            })
            .expect("No camera entity found in world");

        let (camera_component, camera_position) = camera_data;

        let camera = Camera2D {
            target: camera_position.0,
            offset: camera_component.offset,
            rotation: camera_component.rotation,
            zoom: camera_component.zoom,
        };

        let mut camera_draw = draw_handle.begin_mode2D(camera);

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
}

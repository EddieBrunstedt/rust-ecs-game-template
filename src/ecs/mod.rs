pub mod components;
pub mod systems;

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type EntityId = u64;

pub struct World {
    entities: Vec<EntityId>,
    components: HashMap<TypeId, Box<dyn Any>>,
    next_entity_id: EntityId,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            next_entity_id: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entities.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.components.insert(
            TypeId::of::<HashMap<EntityId, T>>(),
            Box::new(HashMap::<EntityId, T>::new()),
        );
    }

    pub fn add_component<T: 'static>(&mut self, entity: EntityId, component: T) {
        let components = self
            .components
            .get_mut(&TypeId::of::<HashMap<EntityId, T>>())
            .and_then(|c| c.downcast_mut::<HashMap<EntityId, T>>())
            .expect("Component storage not registered");
        components.insert(entity, component);
    }

    pub fn get_component<T: 'static>(&self, entity: EntityId) -> Option<&T> {
        self.components
            .get(&TypeId::of::<HashMap<EntityId, T>>())
            .and_then(|c| c.downcast_ref::<HashMap<EntityId, T>>())
            .and_then(|components| components.get(&entity))
    }
}

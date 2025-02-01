pub mod components;
pub mod systems;
use components::{Input, Player, Position, Renderable};
use std::collections::HashMap;

pub type EntityId = u64;
pub type PlayerId = u64;

pub struct World {
    // Entities
    entities: Vec<EntityId>,
    next_entity_id: EntityId,
    next_player_id: PlayerId,

    // Components
    inputs: ComponentStorage<Input>,
    players: ComponentStorage<Player>,
    positions: ComponentStorage<Position>,
    renderables: ComponentStorage<Renderable>,
}

pub struct ComponentStorage<T> {
    data: HashMap<EntityId, T>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId, component: T) {
        self.data.insert(entity, component);
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.data.get(&entity)
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            players: ComponentStorage::new(),
            renderables: ComponentStorage::new(),
            positions: ComponentStorage::new(),
            inputs: ComponentStorage::new(),
            next_entity_id: 0,
            next_player_id: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.entities.push(entity_id);
        self.next_entity_id += 1;
        entity_id
    }

    pub fn add_position(&mut self, entity: EntityId, component: Position) {
        self.positions.insert(entity, component);
    }

    pub fn add_input(&mut self, entity: EntityId, component: Input) {
        self.inputs.insert(entity, component);
    }

    pub fn add_renderable(&mut self, entity: EntityId, component: Renderable) {
        self.renderables.insert(entity, component);
    }

    pub fn add_player(&mut self, entity: EntityId, mut component: Player) {
        component.id = self.next_player_id;
        self.players.insert(entity, component);
        self.next_player_id += 1;
    }

    pub fn get_position(&self, entity: EntityId) -> Option<&Position> {
        self.positions.get(entity)
    }

    pub fn get_renderable(&self, entity: EntityId) -> Option<&Renderable> {
        self.renderables.get(entity)
    }

    pub fn get_player(&self, entity: EntityId) -> Option<&Player> {
        self.players.get(entity)
    }

    pub fn get_input(&self, entity: EntityId) -> Option<&Input> {
        self.inputs.get(entity)
    }
}

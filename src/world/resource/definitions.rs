use crate::world::SceneTransition;

/// Responsible for tracking the global game time.
pub struct GameTimeResource {
    pub total: f32,
}

pub struct SceneTransitionResource {
    pub pending_transition: Option<SceneTransition>,
}

impl SceneTransitionResource {
    pub fn queue_transition(&mut self, transition: SceneTransition) {
        self.pending_transition = Some(transition);
    }
}

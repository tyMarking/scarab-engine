use scarab_engine::{
    gameobject::{entity::HasEntity, Entity},
    rendering::sprite::AnimationStates,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub entity: Entity,
}

impl<'a, 'b: 'a> HasEntity<'a, 'b> for Player {
    fn get_entity(&'b self) -> &'a Entity {
        &self.entity
    }

    fn get_entity_mut(&'b mut self) -> &'a mut Entity {
        &mut self.entity
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerAnimations {
    Idle,
    Run,
}

impl AnimationStates for PlayerAnimations {
    type Viewed = Entity;

    fn next_state(&self, viewed: &Self::Viewed) -> Option<Self> {
        let next = if viewed.get_velocity().magnitude_sq() == 0.0 {
            Self::Idle
        } else {
            Self::Run
        };

        if next != *self {
            Some(next)
        } else {
            None
        }
    }
}

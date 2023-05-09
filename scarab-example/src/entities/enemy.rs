use scarab_engine::{
    gameobject::{entity::HasEntity, Entity},
    HasUuid,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Enemy {
    pub entity: Entity,
}

impl<'a, 'b: 'a> HasEntity<'a, 'b> for Enemy {
    fn get_entity(&'b self) -> &'a Entity {
        &self.entity
    }

    fn get_entity_mut(&'b mut self) -> &'a mut Entity {
        &mut self.entity
    }
}

impl HasUuid for Enemy {
    fn uuid(&self) -> uuid::Uuid {
        self.entity.uuid()
    }
}

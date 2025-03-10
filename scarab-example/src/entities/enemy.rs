use scarab_engine::{
    gameobject::entity::{Entity, HasEntity},
    types::{
        physbox::{HasBox, PhysBox},
        HasUuid,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Enemy {
    pub entity: Entity,
}

impl HasBox for Enemy {
    fn get_box(&self) -> &PhysBox {
        self.entity.get_box()
    }
}

impl HasEntity for Enemy {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

impl HasUuid for Enemy {
    fn uuid(&self) -> uuid::Uuid {
        self.entity.uuid()
    }
}

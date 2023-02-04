use std::sync::mpsc::TryRecvError;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    gameobject::{entity::HasEntity, Entity},
    rendering::View,
    HasUuid, ScarabResult,
};

/// The maximum number of command reads per update
const CHANNEL_READS: usize = 5;

pub type ControlResult<T> = Result<T, ControlError>;

#[derive(Debug, Error, PartialEq)]
pub enum ControlError {
    #[error(transparent)]
    MpscReceive(#[from] TryRecvError),
}

// TODO: Eventually meant to be a trait that can be derived for enums whose
// variants impl HasEntity
pub trait RegisteredEntity: HasUuid {
    fn inner_entity(&self) -> &Entity;

    fn inner_entity_mut(&mut self) -> &mut Entity;

    fn inner_view(&self) -> Box<dyn View<Viewed = Entity> + '_>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityRegistry<E> {
    inner: Vec<E>,
}

impl<E> Default for EntityRegistry<E> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<E> EntityRegistry<E> {
    pub fn register(&mut self, to_register: E) -> ScarabResult<()> {
        self.inner.push(to_register);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get_one(&self, i: usize) -> Option<&E> {
        self.inner.get(i)
    }

    pub fn get_one_mut(&mut self, i: usize) -> Option<&mut E> {
        self.inner.get_mut(i)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, E> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, E> {
        self.inner.iter_mut()
    }
}

impl<E> IntoIterator for EntityRegistry<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, E> IntoIterator for &'a EntityRegistry<E> {
    type Item = &'a E;
    type IntoIter = core::slice::Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, E> IntoIterator for &'a mut EntityRegistry<E> {
    type Item = &'a mut E;
    type IntoIter = core::slice::IterMut<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

pub trait ControlsEntity<'a, 'b: 'a> {
    type ControlledEntity: HasEntity<'a, 'b>;
    type Actions: EntityActions<ControlledEntity = Self::ControlledEntity>;

    /// Takes in a single variant of the Entity's Actions and update's the
    /// entity's model accordingly
    fn do_action(
        &self,
        entity: &mut Self::ControlledEntity,
        action: Self::Actions,
    ) -> ScarabResult<()>;

    /// Returns the next action that should be applied for the entity
    fn next_action(&mut self) -> Option<ControlResult<Self::Actions>>;

    /// Consumes as many commands as possible up to `CHANNEL_READS`
    fn exhaust_channel(&mut self, entity: &mut Self::ControlledEntity) -> ScarabResult<()> {
        for _i in 0..CHANNEL_READS {
            if let Some(action) = self.next_action() {
                self.do_action(entity, action?)?;
            } else {
                break;
            }
        }

        Ok(())
    }
}

pub trait EntityActions {
    type ControlledEntity;
}

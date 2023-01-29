use std::sync::mpsc::TryRecvError;

use thiserror::Error;

use crate::{
    gameobject::{entity::HasEntity, Entity},
    rendering::View,
    ScarabResult,
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
pub trait RegisteredEntity {
    fn inner_entity(&self) -> &Entity;

    fn inner_entity_mut(&mut self) -> &mut Entity;

    fn inner_view(&self) -> Box<dyn View<Viewed = Entity> + '_>;

    /// If 'self' is a controlled entity exhaust its control channel
    /// Returns `Ok(Some(()))` when it's a controlled entity succeeds
    /// `Ok(None)` when it's not a controlled entity
    /// or `Err(_)` when controlling the entity fails
    fn maybe_exhaust_channel<'a, 'b: 'a>(&mut self) -> ScarabResult<Option<()>>;
}

#[derive(Debug)]
pub struct EntityRegistry<E> {
    inner: Vec<E>,
}

impl<E> Default for EntityRegistry<E> {
    fn default() -> Self {
        Self { inner: vec![] }
    }
}

impl<E> EntityRegistry<E> {
    pub fn push(&mut self, to_register: E) {
        self.inner.push(to_register);
    }

    pub fn iter(&self) -> std::slice::Iter<E> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<E> {
        self.inner.iter_mut()
    }
}

impl<E> IntoIterator for EntityRegistry<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, E> IntoIterator for &'a EntityRegistry<E> {
    type Item = &'a E;
    type IntoIter = std::slice::Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, E> IntoIterator for &'a mut EntityRegistry<E> {
    type Item = &'a mut E;
    type IntoIter = std::slice::IterMut<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub trait DoUpdate {
    fn update(&mut self, dt: f64) -> ScarabResult<()>;
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

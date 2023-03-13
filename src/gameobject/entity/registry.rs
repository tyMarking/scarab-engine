use std::sync::mpsc::TryRecvError;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{gameobject::Entity, Camera, HasUuid, PhysicsResult, ScarabResult};

pub type ControlResult<T> = Result<T, ControlError>;

#[derive(Debug, Error, PartialEq)]
pub enum ControlError {
    #[error(transparent)]
    MpscReceive(#[from] TryRecvError),
}

// TODO: Eventually meant to be a trait that can be derived for enums whose
// variants impl HasEntity
/// A trait for types that can be a registered entity
/// This is commonly an enum whose variants are tuples containing Entity and some Entity View
pub trait RegisteredEntity: HasUuid {
    fn inner_entity(&self) -> &Entity;

    fn inner_entity_mut(&mut self) -> &mut Entity;

    fn render(
        &mut self,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()>;
}

/// The registry of all entities that are active in a scene
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityRegistry<E> {
    inner: Vec<E>,
}

impl<E> Default for EntityRegistry<E> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<E: RegisteredEntity> EntityRegistry<E> {
    pub fn register(&mut self, to_register: E) -> PhysicsResult<()> {
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

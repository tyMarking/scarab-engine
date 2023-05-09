use graphics::types::Scalar;
use serde::{Deserialize, Serialize};

use crate::{
    gameobject::HasHealth,
    scene::{PendingEffect, TargetsOthers},
    PhysBox, ScarabResult,
};

use super::registry::RegisteredEntity;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
/// Expresses the readiness of an effect
pub enum Cooldown {
    /// The effect is ready to be used
    #[default]
    Ready,
    /// The effect has a cooldown of `x` seconds remaining
    Cooling(f64),
}

impl Cooldown {
    /// If necessary, reduce the cooldown by dt
    pub fn cool(&mut self, dt: f64) {
        match self {
            Self::Ready => {}
            Self::Cooling(remaining) => {
                *remaining -= dt;
                if *remaining <= 0.0 {
                    *self = Self::Ready;
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// Controls how an entity uses a specific action
pub struct TryAction {
    /// Should the attached entity attempt to use an action on the next update
    pub try_action: bool,
    /// The readiness of the action
    pub cooldown: Cooldown,
}

impl TryAction {
    /// If the cooldown is ready, marks the entity to do the action on the next update
    pub fn maybe_set_doing(&mut self) {
        match self.cooldown {
            Cooldown::Ready => self.try_action = true,
            Cooldown::Cooling(_) => {}
        }
    }

    /// Returns whether the action should be done on the next tick
    pub fn should_do(&mut self, cooldown: Cooldown) -> bool {
        if self.try_action {
            self.try_action = false;
            self.cooldown = cooldown;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
/// Represents an attack that does a raw amount of damage on an entity
pub struct BasicAttack {
    damage: Scalar,
}

impl BasicAttack {
    /// Sets the raw damage for this attack
    pub fn new(damage: Scalar) -> Self {
        Self { damage }
    }

    /// Transforms self into a pending effect so it can be applied on the next tick
    pub fn into_pending_effect<E: RegisteredEntity>(
        &self,
        source_index: usize,
        target_area: PhysBox,
    ) -> PendingEffect<E> {
        PendingEffect {
            source: Some((source_index, false).into()),
            target_area,
            effect: Box::new(*self),
        }
    }
}

impl<E: RegisteredEntity> TargetsOthers<E> for BasicAttack {
    fn apply_effect(&mut self, target: &mut E) -> ScarabResult<bool> {
        target
            .inner_entity_mut()
            .get_health_mut()
            .raw_damage(self.damage);
        Ok(false)
    }

    fn update_src(&mut self, _src: &mut E) -> ScarabResult<()> {
        Ok(())
    }
}

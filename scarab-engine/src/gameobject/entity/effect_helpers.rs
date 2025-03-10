use graphics::types::Scalar;
use serde::{Deserialize, Serialize};

use super::registry::RegisteredEntity;
use crate::{
    gameobject::HasHealth,
    scene::{PendingEffect, TargetsOthers},
    types::physbox::PhysBox,
    ScarabResult,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
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

impl From<Cooldown> for f64 {
    fn from(value: Cooldown) -> Self {
        match value {
            Cooldown::Ready => 0.0,
            Cooldown::Cooling(x) => x,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_action_sets_doing_when_ready() {
        let mut try_action = TryAction {
            try_action: false,
            cooldown: Cooldown::Ready,
        };

        try_action.maybe_set_doing();

        assert!(try_action.try_action);

        try_action.maybe_set_doing();
        assert!(try_action.try_action);
    }

    #[test]
    fn try_action_doesnt_set_doing_when_cooling() {
        let mut try_action = TryAction {
            try_action: false,
            cooldown: Cooldown::Cooling(0.0),
        };

        try_action.maybe_set_doing();
        assert!(!try_action.try_action);
    }

    #[test]
    fn try_action_should_do_only_when_set_to_do() {
        let mut try_action = TryAction {
            try_action: false,
            cooldown: Cooldown::Cooling(0.0),
        };

        assert!(!try_action.should_do(Cooldown::Cooling(0.0)));

        try_action.cooldown = Cooldown::Ready;
        assert!(!try_action.should_do(Cooldown::Cooling(0.0)));

        try_action.try_action = true;
        assert!(try_action.should_do(Cooldown::Cooling(0.0)));

        try_action.try_action = true;
        try_action.cooldown = Cooldown::Ready;
        assert!(try_action.should_do(Cooldown::Cooling(0.0)));
    }

    #[test]
    fn try_action_should_do_resets_cooldown_only_when_true() {
        let mut try_action = TryAction {
            try_action: false,
            cooldown: Cooldown::Cooling(0.0),
        };

        assert!(!try_action.should_do(Cooldown::Cooling(5.0)));
        assert_eq!(try_action.cooldown, Cooldown::Cooling(0.0));

        try_action.try_action = true;
        try_action.cooldown = Cooldown::Ready;
        assert!(try_action.should_do(Cooldown::Cooling(5.0)));
        assert_eq!(try_action.cooldown, Cooldown::Cooling(5.0));
    }

    #[test]
    fn cooldown_cool_reduces_remaining_time() {
        let start = 5.0;
        let reduction = 2.3;
        let mut cooldown = Cooldown::Cooling(start);

        cooldown.cool(reduction);
        assert_eq!(cooldown, Cooldown::Cooling(start - reduction))
    }

    #[test]
    fn cooldown_cool_marks_ready_when_zero_or_less() {
        let start = 5.0;
        let mut cooldown = Cooldown::Cooling(start);

        cooldown.cool(start);
        assert_eq!(cooldown, Cooldown::Ready);

        cooldown = Cooldown::Cooling(start);
        cooldown.cool(start + 1.0);
        assert_eq!(cooldown, Cooldown::Ready);
    }

    #[test]
    fn cooldown_cool_doesnt_change_when_ready() {
        let mut cooldown = Cooldown::Ready;
        cooldown.cool(5.0);
        assert_eq!(cooldown, Cooldown::Ready);
    }
}

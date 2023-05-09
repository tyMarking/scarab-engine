use scarab_engine::{
    gameobject::{
        entity::{
            effect_helpers::{BasicAttack, Cooldown, TryAction},
            HasEntity,
        },
        Entity,
    },
    rendering::sprite::AnimationStates,
    scene::GameTickArgs,
    HasBox, HasUuid, ScarabResult,
};
use serde::{Deserialize, Serialize};
use shapes::Point;

use super::ExampleEntities;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub entity: Entity,
    attack: (TryAction, BasicAttack, f64),
}

impl Player {
    pub fn new(entity: Entity, damage: f64, cooldown: f64) -> Self {
        Self {
            entity,
            attack: (TryAction::default(), BasicAttack::new(damage), cooldown),
        }
    }

    pub fn attack(&mut self) {
        self.attack.0.maybe_set_doing();
    }

    pub fn game_tick(
        &mut self,
        this_idx: usize,
        args: &mut GameTickArgs<ExampleEntities>,
    ) -> ScarabResult<()> {
        self.entity.game_tick(args)?;

        self.attack.0.cooldown.cool(args.dt);

        if self.attack.0.should_do(Cooldown::Cooling(self.attack.2)) {
            let mut target_area = self.entity.get_box().clone();
            let size = self.entity.get_box().size();
            let _ = target_area.set_size([size.w * 2.0, size.h * 2.0].into());
            target_area.set_pos(*self.entity.get_box().pos() - Point::from([size.w, size.h]));
            args.pending_attacks
                .push(self.attack.1.into_pending_effect(this_idx, target_area));
        }

        Ok(())
    }
}

impl<'a, 'b: 'a> HasEntity<'a, 'b> for Player {
    fn get_entity(&'b self) -> &'a Entity {
        &self.entity
    }

    fn get_entity_mut(&'b mut self) -> &'a mut Entity {
        &mut self.entity
    }
}

impl HasUuid for Player {
    fn uuid(&self) -> uuid::Uuid {
        self.entity.uuid()
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

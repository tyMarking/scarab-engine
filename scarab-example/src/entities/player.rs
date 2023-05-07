use graphics::types::Scalar;
use scarab_engine::{
    gameobject::{
        entity::{
            registry::{GameTickArgs, RegisteredEntity},
            HasEntity,
        },
        Entity, HasHealth,
    },
    rendering::sprite::AnimationStates,
    scene::{Attack, PendingAttack},
    HasBox, HasUuid, ScarabResult,
};
use serde::{Deserialize, Serialize};
use shapes::Point;

use super::ExampleEntities;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub entity: Entity,
    try_attack: TryAttack,
}

impl Player {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            try_attack: TryAttack::default(),
        }
    }

    pub fn attack(&mut self) {
        self.try_attack.maybe_set_attacking();
    }

    pub fn game_tick(
        &mut self,
        this_idx: usize,
        args: &mut GameTickArgs<ExampleEntities>,
    ) -> ScarabResult<()> {
        self.entity.game_tick(args)?;

        self.try_attack.cooldown.cool(args.dt);

        if self.try_attack.maybe_attack(Cooldown::Cooling(2.0)) {
            let mut target_area = self.entity.get_box().clone();
            let size = self.entity.get_box().size();
            let _ = target_area.set_size([size.w * 2.0, size.h * 2.0].into());
            target_area.set_pos(*self.entity.get_box().pos() - Point::from([size.w, size.h]));
            args.pending_attacks.push(PendingAttack {
                src_idx: this_idx,
                can_target_src: false,
                target_area,
                attack: Box::new(BasicAttack::new(2.0)),
            })
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

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub enum Cooldown {
    #[default]
    Ready,
    Cooling(f64),
}

impl Cooldown {
    fn cool(&mut self, dt: f64) {
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
pub struct TryAttack {
    // Should the attached entity attempt to attack on the next update
    try_attack: bool,
    cooldown: Cooldown,
}

impl TryAttack {
    /// If the cooldown is ready, marks the entity to attack on the next update
    fn maybe_set_attacking(&mut self) {
        match self.cooldown {
            Cooldown::Ready => self.try_attack = true,
            Cooldown::Cooling(_) => {}
        }
    }

    fn maybe_attack(&mut self, cooldown: Cooldown) -> bool {
        if self.try_attack {
            self.try_attack = false;
            self.cooldown = cooldown;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct BasicAttack {
    damage: Scalar,
}

impl BasicAttack {
    fn new(damage: Scalar) -> Self {
        Self { damage }
    }
}

impl Attack<ExampleEntities> for BasicAttack {
    fn do_attack(&mut self, target: &mut ExampleEntities) -> ScarabResult<bool> {
        println!("Attacked target: {:?}", target);

        target
            .inner_entity_mut()
            .get_health_mut()
            .raw_damage(self.damage);
        Ok(false)
    }

    fn update_src(&mut self, _src: &mut ExampleEntities) -> ScarabResult<()> {
        Ok(())
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

use core::marker::PhantomData;

use piston::Input;
use scarab_engine::{
    input::{ButtonBinding, InputBinding, InputRegistry, UnitAxis2dBinding},
    types::Velocity,
    ScarabResult,
};
use serde::{Deserialize, Serialize};

use crate::{debug::DebugOptions, entities::Player};

#[derive(Debug, Serialize, Deserialize)]
pub enum GameInputActions {
    SetPlayerMovement(Velocity),
    Attack,
    Nop,
}

#[derive(Serialize, Deserialize)]
pub struct GameInputs<'a> {
    pub move_binding: Option<UnitAxis2dBinding>,
    pub attack_binding: Option<ButtonBinding>,
    phantom: PhantomData<&'a u8>,
}

impl<'a> InputRegistry for GameInputs<'a> {
    type InputActions = GameInputActions;
    type InputTarget = Player;

    fn do_input_action(
        &self,
        action: Self::InputActions,
        target: &mut Self::InputTarget,
    ) -> ScarabResult<()> {
        match action {
            GameInputActions::SetPlayerMovement(vel) => {
                target
                    .entity
                    .set_velocity(vel * target.entity.get_max_velocity());
            }
            GameInputActions::Attack => {
                target.attack();
            }
            GameInputActions::Nop => {}
        }

        Ok(())
    }

    fn map_input_to_action(&mut self, input: &Input) -> Option<Self::InputActions> {
        self.move_binding
            .as_mut()
            .map(|binding| binding.maybe_to_action(input))
            .flatten()
            .map(|velocity| GameInputActions::SetPlayerMovement(velocity.into()))
            .or_else(|| {
                self.attack_binding
                    .as_mut()
                    .map(|binding| binding.maybe_to_action(input))
                    .flatten()
                    .map(|state| {
                        if state {
                            Some(GameInputActions::Attack)
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
    }
}

impl<'a> GameInputs<'a> {
    pub fn new() -> Self {
        Self {
            move_binding: None,
            attack_binding: None,
            phantom: PhantomData::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AppInputActions {
    ToggleDebugEntityCollisionBoxes,
    ToggleDebugEntityHealth,
    ToggleDebugFieldCollisionBoxes,
    ToggleDebugAttackCooldowns,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppInputs {
    pub toggle_debug_entity_collision_boxes: Option<ButtonBinding>,
    pub toggle_debug_entity_health: Option<ButtonBinding>,
    pub toggle_debug_field_collision_boxes: Option<ButtonBinding>,
    pub toggle_debug_attack_cooldowns: Option<ButtonBinding>,
}

impl InputRegistry for AppInputs {
    type InputActions = AppInputActions;
    type InputTarget = DebugOptions;

    fn do_input_action(
        &self,
        action: Self::InputActions,
        target: &mut Self::InputTarget,
    ) -> ScarabResult<()> {
        match action {
            AppInputActions::ToggleDebugEntityCollisionBoxes => {
                target.entity_collision_boxes = !target.entity_collision_boxes;
            }
            AppInputActions::ToggleDebugEntityHealth => {
                target.entity_health = !target.entity_health;
            }
            AppInputActions::ToggleDebugFieldCollisionBoxes => {
                target.field_collision_boxes = !target.field_collision_boxes;
            }
            AppInputActions::ToggleDebugAttackCooldowns => {
                target.attack_cooldowns = !target.attack_cooldowns;
            }
        }

        Ok(())
    }

    fn map_input_to_action(&mut self, input: &Input) -> Option<Self::InputActions> {
        self.toggle_debug_entity_collision_boxes
            .as_mut()
            .map(|binding| binding.maybe_to_action(input))
            .flatten()
            .map(|state| {
                if state {
                    Some(AppInputActions::ToggleDebugEntityCollisionBoxes)
                } else {
                    None
                }
            })
            .flatten()
            .or_else(|| {
                // TODO! this little Button -> do if pressed would be a good macro
                self.toggle_debug_entity_health
                    .as_mut()
                    .map(|binding| binding.maybe_to_action(input))
                    .flatten()
                    .map(|state| {
                        if state {
                            Some(AppInputActions::ToggleDebugEntityHealth)
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
            .or_else(|| {
                self.toggle_debug_field_collision_boxes
                    .as_mut()
                    .map(|binding| binding.maybe_to_action(&input))
                    .flatten()
                    .map(|state| {
                        if state {
                            Some(AppInputActions::ToggleDebugFieldCollisionBoxes)
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
            .or_else(|| {
                self.toggle_debug_attack_cooldowns
                    .as_mut()
                    .map(|binding| binding.maybe_to_action(&input))
                    .flatten()
                    .map(|state| {
                        if state {
                            Some(AppInputActions::ToggleDebugAttackCooldowns)
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
    }
}

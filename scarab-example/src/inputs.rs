use core::marker::PhantomData;

use piston::Input;
use scarab_engine::{
    input::{ButtonBinding, InputBinding, InputRegistry, UnitAxis2dBinding},
    ScarabResult, Velocity,
};
use serde::{Deserialize, Serialize};

use crate::entities::Player;

#[derive(Debug, Serialize, Deserialize)]
pub enum InputActions {
    SetPlayerMovement(Velocity),
    Attack,
    Nop,
}

#[derive(Serialize, Deserialize)]
pub struct Inputs<'a> {
    move_binding: Option<UnitAxis2dBinding>,
    attack_binding: Option<ButtonBinding>,
    phantom: PhantomData<&'a u8>,
}

impl<'a> InputRegistry for Inputs<'a> {
    type InputActions = InputActions;
    type InputTarget = Player;

    fn do_input_action(
        &self,
        action: Self::InputActions,
        target: &mut Self::InputTarget,
    ) -> ScarabResult<()> {
        match action {
            InputActions::SetPlayerMovement(vel) => {
                target
                    .entity
                    .set_velocity(vel * target.entity.get_max_velocity());
            }
            InputActions::Attack => {
                target.attack();
            }
            InputActions::Nop => {}
        }

        Ok(())
    }

    fn map_input_to_action(&mut self, input: Input) -> Option<Self::InputActions> {
        self.move_binding
            .as_mut()
            .map(|binding| binding.maybe_to_action(&input))
            .flatten()
            .map(|velocity| InputActions::SetPlayerMovement(velocity.into()))
            .or_else(|| {
                self.attack_binding
                    .as_mut()
                    .map(|binding| binding.maybe_to_action(&input))
                    .flatten()
                    .map(|state| {
                        if state {
                            InputActions::Attack
                        } else {
                            InputActions::Nop
                        }
                    })
            })
    }
}

impl<'a> Inputs<'a> {
    pub fn new() -> Self {
        Self {
            move_binding: None,
            attack_binding: None,
            phantom: PhantomData::default(),
        }
    }

    pub fn bind_movement(&mut self, binding: UnitAxis2dBinding) {
        self.move_binding.replace(binding);
    }

    pub fn bind_attack(&mut self, binding: ButtonBinding) {
        self.attack_binding.replace(binding);
    }
}

use core::marker::PhantomData;

use piston::Input;
use scarab_engine::{
    gameobject::entity::registry::RegisteredEntity,
    input::{Axis2dBinding, InputRegistry},
    ScarabResult, Velocity,
};
use serde::{Deserialize, Serialize};

use crate::entities::ExampleEntities;

#[derive(Debug, Serialize, Deserialize)]
pub enum InputActions {
    SetPlayerMovement(Velocity),
    Nop,
}

#[derive(Serialize, Deserialize)]
pub struct Inputs<'a> {
    move_binding: Option<Axis2dBinding>,
    phantom: PhantomData<&'a u32>,
}

impl<'a> InputRegistry for Inputs<'a> {
    type InputActions = InputActions;
    // TODO! this should ideally only target player
    type InputTarget = ExampleEntities;

    fn do_input_action(
        &self,
        action: Self::InputActions,
        target: &mut Self::InputTarget,
    ) -> ScarabResult<()> {
        match action {
            InputActions::SetPlayerMovement(vel) => {
                let entity = target.inner_entity_mut();
                entity.set_velocity(vel * entity.get_max_velocity());
            }
            InputActions::Nop => {}
        }

        Ok(())
    }

    fn map_input_to_action(&mut self, input: Input) -> Option<Self::InputActions> {
        match input {
            Input::Button(args) => self
                .move_binding
                .as_mut()
                .map(|binding| binding.maybe_to_action(args))
                .flatten()
                .map(|vel| InputActions::SetPlayerMovement(vel.into())),
            _ => None,
        }
    }
}

impl<'a> Inputs<'a> {
    pub fn new() -> Self {
        Self {
            move_binding: None,
            phantom: PhantomData::default(),
        }
    }

    pub fn bind_movement(&mut self, binding: Axis2dBinding) {
        self.move_binding.replace(binding);
    }
}

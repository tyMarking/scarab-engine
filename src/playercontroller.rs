use std::{
    collections::HashMap,
    sync::mpsc::{SendError, Sender},
};

use piston::{Button, ButtonState, Input};
use uuid::Uuid;

use crate::{gameobject::entity::EntityControls, Velocity};

use thiserror::Error;

pub type InputResult<T> = Result<T, InputError>;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("No axis handler with the given id '{0}' exists")]
    NoAxisRegistered(Uuid),
    #[error("Channel send operation failed")]
    ChannelSendFail,
}

impl<T: Send> From<SendError<T>> for InputError {
    fn from(_: SendError<T>) -> Self {
        InputError::ChannelSendFail
    }
}

// How to structure the player controls......
// i want it to be generic as possible so the consumer can
// take this and generically construct it with any inputs and give it to any
// controlled object with the handler functions

// Input types:
// generic keyboard (mouse buttons included) (single mouse button)
// axis (a set of inputs corresponding to a vec2)
// mouse (pointer)
// mouse scroll

#[derive(Debug)]
pub struct InputController {
    sender: Sender<EntityControls>,
    button_bindings: HashMap<Button, InputHandler<ButtonState>>,
    // This poses some serious problems for removing an axis handler....
    // would have to modify potentiall all of the values with the updated index
    axis_bindings: HashMap<Button, (Uuid, Axis2dDirection)>,
    axis_handlers: HashMap<Uuid, InputHandler<Axis2d>>,
}

impl InputController {
    pub fn new(sender: Sender<EntityControls>) -> Self {
        Self {
            sender,
            button_bindings: HashMap::new(),
            axis_bindings: HashMap::new(),
            axis_handlers: HashMap::new(),
        }
    }

    pub fn bind_button(
        &mut self,
        button: Button,
        handler: InputHandler<ButtonState>,
    ) -> InputResult<()> {
        self.button_bindings.insert(button, handler);
        Ok(())
    }

    pub fn unbind_button(&mut self, button: Button) -> Option<InputHandler<ButtonState>> {
        self.button_bindings.remove(&button)
    }

    /// Returns an ID for unbinding/editing the axis handler
    pub fn bind_axis(
        &mut self,
        pos_x: Button,
        neg_x: Button,
        pos_y: Button,
        neg_y: Button,
        handler: InputHandler<Axis2d>,
    ) -> InputResult<Uuid> {
        let id = Uuid::new_v4();
        self.axis_handlers.insert(id, handler);
        self.axis_bindings
            .insert(pos_x, (id, Axis2dDirection::PosX));
        self.axis_bindings
            .insert(neg_x, (id, Axis2dDirection::NegX));
        self.axis_bindings
            .insert(pos_y, (id, Axis2dDirection::PosY));
        self.axis_bindings
            .insert(neg_y, (id, Axis2dDirection::NegY));
        Ok(id)
    }

    pub fn unbind_axis(&mut self, id: Uuid) -> InputResult<()> {
        if self.axis_handlers.contains_key(&id) {
            self.axis_handlers.remove(&id);
            self.axis_bindings
                .retain(|_, (binding_id, _)| binding_id != &id);
            Ok(())
        } else {
            Err(InputError::NoAxisRegistered(id))
        }
    }

    // TODO: unbind axis handlerssss

    pub fn input_event(&mut self, input: &Input) -> InputResult<()> {
        match input {
            Input::Button(args) => {
                if let Some(handler) = self.button_bindings.get_mut(&args.button) {
                    handler.set_state(args.state);
                    self.sender.send(handler.control_binding())?;
                } else if let Some((id, dir)) = self.axis_bindings.get(&args.button) {
                    let handler = self
                        .axis_handlers
                        .get_mut(id)
                        .map_or_else(|| Err(InputError::NoAxisRegistered(*id)), |h| Ok(h))?;
                    handler.state.set_axis_button(args.state, dir);
                    self.sender.send(handler.control_binding())?;
                }
            }
            _ => {}
        };
        Ok(())
    }
}

// Cases to implement
//     ButtonState(Button),
//     ButtonToggle(Button),
//     Axis2d(Axis2dButtons),
//     MousePos([f64; 2]),
//     MouseScroll([f64; 2]),
// }

#[derive(Debug)]
pub struct InputHandler<T: Copy> {
    state: T,
    to_control_binding: fn(T) -> EntityControls,
}

impl<T: Copy> InputHandler<T> {
    pub fn new(state: T, handler: fn(T) -> EntityControls) -> Self {
        Self {
            state,
            to_control_binding: handler,
        }
    }

    fn set_state(&mut self, new_state: T) {
        self.state = new_state;
    }

    fn control_binding(&self) -> EntityControls {
        let foo = self.to_control_binding;
        foo(self.state)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Axis2d {
    pos_x: f64,
    neg_x: f64,
    pos_y: f64,
    neg_y: f64,
}

impl Axis2d {
    fn set_axis_button(&mut self, button: ButtonState, dir: &Axis2dDirection) {
        let val = match button {
            ButtonState::Press => 1.0,
            ButtonState::Release => 0.0,
        };
        self.set_axis(val, dir)
    }

    fn set_axis(&mut self, val: f64, dir: &Axis2dDirection) {
        match dir {
            Axis2dDirection::PosX => self.pos_x = val,
            Axis2dDirection::NegX => self.neg_x = val,
            Axis2dDirection::PosY => self.pos_y = val,
            Axis2dDirection::NegY => self.neg_y = val,
        }
    }
}

impl From<Axis2d> for [f64; 2] {
    fn from(val: Axis2d) -> Self {
        [val.pos_x - val.neg_x, val.pos_y - val.neg_y]
    }
}

impl From<Axis2d> for Velocity {
    fn from(val: Axis2d) -> Self {
        Velocity {
            x: val.pos_x - val.neg_x,
            y: val.pos_y - val.neg_y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Axis2dDirection {
    PosX,
    NegX,
    PosY,
    NegY,
}

#[cfg(test)]
mod test {
    // Test cases:
    // normal button presses,
    // axis button presses,
}

use piston::{Button, ButtonArgs, ButtonState, Input};
use serde::{Deserialize, Serialize};

use crate::ScarabResult;

/// A trait for types that handle user inputs.
/// User input handling is split into two stages: mapping input to action and performing the action
/// This division is intended to allow for a more intuitive divide between parsing the inputs and
/// actually doing the things they're intended to do.
pub trait InputRegistry {
    /// The different actions that the registry can handle
    type InputActions;
    /// What the input action should act upon
    type InputTarget;

    /// Modifies the `InputTarget` as necessary according to the given action
    /// i.e. a movement action would set an entity's velocity
    fn do_input_action(
        &self,
        action: Self::InputActions,
        target: &mut Self::InputTarget,
    ) -> ScarabResult<()>;

    /// Given an event input (i.e. key press, mouse movement, etc.) turns it into an instance of `InputActions`
    fn map_input_to_action(&mut self, input: Input) -> Option<Self::InputActions>;
}

/// Represents a type of input binding and how it is transformed into an action argument
pub trait InputBinding {
    /// The type of value that this input can produce (i.e. [bool])
    type ActionArg;

    /// If the given input matches this binding, returns the result corresponding to the input
    fn maybe_to_action(&mut self, input: &Input) -> Option<Self::ActionArg>;
}

// TODO! a piston button can already be a "ControllerHat" which is a D-pad already,
// have to figure out how to reconcile that
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A binding between 4 keyboard buttons and a representation of a 2-axis D-pad
pub struct Axis2dBinding {
    pos_x: (Button, f64),
    neg_x: (Button, f64),
    pos_y: (Button, f64),
    neg_y: (Button, f64),
}

impl Axis2dBinding {
    /// Makes a new instance of self for the given buttons
    pub fn new(pos_x: Button, pos_y: Button, neg_x: Button, neg_y: Button) -> Self {
        Self {
            pos_x: (pos_x, 0.0),
            pos_y: (pos_y, 0.0),
            neg_x: (neg_x, 0.0),
            neg_y: (neg_y, 0.0),
        }
    }

    /// Sets the value for the corresponding direction to 1 or 0 depending on the button state
    fn set_axis_button(&mut self, button: ButtonState, dir: Axis2dDirection) {
        let val = match button {
            ButtonState::Press => 1.0,
            ButtonState::Release => 0.0,
        };
        self.set_axis(val, dir)
    }

    fn set_axis(&mut self, val: f64, dir: Axis2dDirection) {
        match dir {
            Axis2dDirection::PosX => self.pos_x.1 = val,
            Axis2dDirection::NegX => self.neg_x.1 = val,
            Axis2dDirection::PosY => self.pos_y.1 = val,
            Axis2dDirection::NegY => self.neg_y.1 = val,
        }
    }

    fn maybe_direction_from_button(&self, args: &ButtonArgs) -> Option<Axis2dDirection> {
        if args.button == self.pos_x.0 {
            Some(Axis2dDirection::PosX)
        } else if args.button == self.pos_y.0 {
            Some(Axis2dDirection::PosY)
        } else if args.button == self.neg_x.0 {
            Some(Axis2dDirection::NegX)
        } else if args.button == self.neg_y.0 {
            Some(Axis2dDirection::NegY)
        } else {
            None
        }
    }
}

impl InputBinding for Axis2dBinding {
    type ActionArg = [f64; 2];

    fn maybe_to_action(&mut self, input: &Input) -> Option<Self::ActionArg> {
        if let Input::Button(args) = input {
            if let Some(dir) = self.maybe_direction_from_button(&args) {
                self.set_axis_button(args.state, dir);
                return Some(self.into());
            }
        };

        None
    }
}

impl From<Axis2dBinding> for [f64; 2] {
    fn from(val: Axis2dBinding) -> Self {
        [val.pos_x.1 - val.neg_x.1, val.pos_y.1 - val.neg_y.1]
    }
}

impl From<&Axis2dBinding> for [f64; 2] {
    fn from(val: &Axis2dBinding) -> Self {
        [val.pos_x.1 - val.neg_x.1, val.pos_y.1 - val.neg_y.1]
    }
}

impl From<&mut Axis2dBinding> for [f64; 2] {
    fn from(val: &mut Axis2dBinding) -> Self {
        [val.pos_x.1 - val.neg_x.1, val.pos_y.1 - val.neg_y.1]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// A cardinal direction on a 2-d coorinate plane
pub enum Axis2dDirection {
    /// Positive-X direction (left)
    PosX,
    /// Negative-X direction (right)
    NegX,
    /// Positive-Y direction (down)
    PosY,
    /// Negative-Y direction (up)
    NegY,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An input binding that is true iff the corresponding button is `high_state`
pub struct ButtonBinding {
    high_state: ButtonState,
    button: Button,
}

impl ButtonBinding {
    /// Makes a new instance of self for the given button and state which it should be true
    pub fn new(high_state: ButtonState, button: Button) -> Self {
        Self { high_state, button }
    }
}

impl InputBinding for ButtonBinding {
    type ActionArg = bool;

    fn maybe_to_action(&mut self, input: &Input) -> Option<Self::ActionArg> {
        if let Input::Button(args) = input {
            if args.button == self.button {
                return Some(args.state == self.high_state);
            }
        };

        None
    }
}

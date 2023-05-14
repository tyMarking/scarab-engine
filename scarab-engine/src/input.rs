use piston::{
    Button, ButtonArgs, ButtonState, ControllerButton, ControllerHat, HatState, Input, Key,
    MouseButton,
};
use serde::{Deserialize, Serialize};

use crate::{types::ROOT_2, ScarabResult};

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
    fn map_input_to_action(&mut self, input: &Input) -> Option<Self::InputActions>;
}

/// Represents a type of input binding and how it is transformed into an action argument
pub trait InputBinding {
    /// The type of value that this input can produce (i.e. [bool])
    type ActionArg;

    /// If the given input matches this binding, returns the result corresponding to the input
    fn maybe_to_action(&mut self, input: &Input) -> Option<Self::ActionArg>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An enum for button types that consist on only 1 button (b/c [piston::Buton] also includes physical dpads)
pub enum SingleButton {
    /// A button on a keyboard
    Keyboard(Key),
    /// A button on a mouse (i.e. left/right click)
    Mouse(MouseButton),
    /// A button on a console controller
    Controller(ControllerButton),
}

impl From<SingleButton> for Button {
    fn from(value: SingleButton) -> Self {
        match value {
            SingleButton::Keyboard(key) => Button::Keyboard(key),
            SingleButton::Mouse(mouse_button) => Button::Mouse(mouse_button),
            SingleButton::Controller(controller_button) => Button::Controller(controller_button),
        }
    }
}

impl PartialEq<Button> for SingleButton {
    fn eq(&self, other: &Button) -> bool {
        match other {
            Button::Hat(_) => false,
            o => o == &Button::from(self.clone()),
        }
    }
}

impl PartialEq<SingleButton> for Button {
    fn eq(&self, other: &SingleButton) -> bool {
        other == self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A generic input binding representing 2-d inputs that can be anywhere on/in the unit circle
pub enum UnitAxis2dBinding {
    /// A unit circle binding made up of a logical d-pad. This can only generate inputs on the circle every 45 degrees.
    LogicalDpad(LogicalDpad),
    // TODO: add support for joysticks etc.
}

impl InputBinding for UnitAxis2dBinding {
    type ActionArg = [f64; 2];

    fn maybe_to_action(&mut self, input: &Input) -> Option<Self::ActionArg> {
        match self {
            UnitAxis2dBinding::LogicalDpad(dpad) => dpad.maybe_to_action(input),
        }
    }
}

impl From<LogicalDpad> for UnitAxis2dBinding {
    fn from(value: LogicalDpad) -> Self {
        UnitAxis2dBinding::LogicalDpad(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An input that logically represents a "D-pad"
pub enum LogicalDpad {
    /// Virtually assembles separate buttons into a D-pad (i.e. WASD on a keyboard)
    VirtualDpad(VirtualDpad),
    /// A physical D-pad (i.e. on a console controller)
    PhysicalDpad(ControllerHat),
}

impl InputBinding for LogicalDpad {
    type ActionArg = [f64; 2];

    fn maybe_to_action(&mut self, input: &Input) -> Option<Self::ActionArg> {
        match self {
            LogicalDpad::VirtualDpad(dpad) => dpad.maybe_to_action(input),
            LogicalDpad::PhysicalDpad(dpad) => dpad.maybe_to_action(input),
        }
    }
}

impl From<VirtualDpad> for LogicalDpad {
    fn from(value: VirtualDpad) -> Self {
        LogicalDpad::VirtualDpad(value)
    }
}

impl From<ControllerHat> for LogicalDpad {
    fn from(value: ControllerHat) -> Self {
        LogicalDpad::PhysicalDpad(value)
    }
}

impl InputBinding for ControllerHat {
    type ActionArg = [f64; 2];

    fn maybe_to_action(&mut self, input: &Input) -> Option<Self::ActionArg> {
        if let Input::Button(args) = input {
            if let Button::Hat(dpad) = args.button {
                if dpad.id == self.id && dpad.which == self.which {
                    return Some(hat_state_to_action_arg(dpad.state));
                }
            }
        }
        None
    }
}

/// Up is -y, Left is +x etc.
fn hat_state_to_action_arg(value: HatState) -> [f64; 2] {
    match value {
        HatState::Centered => [0.0, 0.0],
        HatState::Up => [0.0, -1.0],
        HatState::Right => [-1.0, 0.0],
        HatState::Down => [0.0, 1.0],
        HatState::Left => [1.0, 0.0],
        HatState::RightUp => [-*ROOT_2, -*ROOT_2],
        HatState::RightDown => [-*ROOT_2, *ROOT_2],
        HatState::LeftUp => [*ROOT_2, -*ROOT_2],
        HatState::LeftDown => [*ROOT_2, *ROOT_2],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A logical representation of a 2-axis D-pad
pub struct VirtualDpad {
    pos_x: (SingleButton, f64),
    neg_x: (SingleButton, f64),
    pos_y: (SingleButton, f64),
    neg_y: (SingleButton, f64),
}

impl VirtualDpad {
    /// Makes a new instance of self for the given buttons
    pub fn new(
        pos_x: SingleButton,
        pos_y: SingleButton,
        neg_x: SingleButton,
        neg_y: SingleButton,
    ) -> Self {
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
        if self.pos_x.0 == args.button {
            Some(Axis2dDirection::PosX)
        } else if self.pos_y.0 == args.button {
            Some(Axis2dDirection::PosY)
        } else if self.neg_x.0 == args.button {
            Some(Axis2dDirection::NegX)
        } else if self.neg_y.0 == args.button {
            Some(Axis2dDirection::NegY)
        } else {
            None
        }
    }
}

impl InputBinding for VirtualDpad {
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

impl From<VirtualDpad> for [f64; 2] {
    fn from(val: VirtualDpad) -> Self {
        let mut x = val.pos_x.1 - val.neg_x.1;
        let mut y = val.pos_y.1 - val.neg_y.1;
        if x != 0.0 && y != 0.0 {
            x *= *ROOT_2;
            y *= *ROOT_2;
        }
        [x, y]
    }
}

impl From<&VirtualDpad> for [f64; 2] {
    fn from(val: &VirtualDpad) -> Self {
        [val.pos_x.1 - val.neg_x.1, val.pos_y.1 - val.neg_y.1]
    }
}

impl From<&mut VirtualDpad> for [f64; 2] {
    fn from(val: &mut VirtualDpad) -> Self {
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
    button: SingleButton,
}

impl ButtonBinding {
    /// Makes a new instance of self for the given button and state which it should be true
    pub fn new(high_state: ButtonState, button: SingleButton) -> Self {
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

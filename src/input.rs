use std::sync::mpsc::SendError;

use piston::Input;
use thiserror::Error;
use uuid::Uuid;

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

pub trait RegisteredInput {
    fn send_input(&mut self, input: &Input) -> InputResult<()>;
}

pub struct InputRegistry<I> {
    inner: Vec<I>,
}

impl<I> Default for InputRegistry<I> {
    fn default() -> Self {
        Self { inner: vec![] }
    }
}

impl<I: RegisteredInput> InputRegistry<I> {
    pub fn push(&mut self, to_register: I) {
        self.inner.push(to_register);
    }

    pub fn send_input(&mut self, input: &Input) -> InputResult<()> {
        for registered_input in &mut self.inner {
            registered_input.send_input(input)?;
        }
        Ok(())
    }
}

use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::{Gamestate, ScarabResult, VecNum};

/// The maximum number of command reads per update
const CHANNEL_READS: usize = 5;

pub trait DoUpdate {
    fn update(&mut self, dt: f64) -> ScarabResult<()>;
}

pub trait UpdateChannel<N: VecNum, T: Send> {
    // Call for updates that should happen on every game tick
    fn game_tick(&mut self, gamestate: &Gamestate<N>, dt: f64) -> ScarabResult<()>;

    fn get_sender(&self) -> Sender<T>;

    /// Reads one command from the channel and performs the corresponding action
    fn consume_channel(&mut self) -> Option<Result<(), TryRecvError>>;

    /// Consumes as many commands as possible up to `CHANNEL_READS`
    fn exhaust_channel(&mut self) -> ScarabResult<()> {
        for _i in 0..CHANNEL_READS {
            if let Some(res) = self.consume_channel() {
                res?;
            } else {
                break;
            }
        }

        Ok(())
    }
}

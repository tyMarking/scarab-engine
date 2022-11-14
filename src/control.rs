use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::{ScarabResult, VecNum};

const CHANNEL_READS: usize = 5;

pub trait DoUpdate {
    fn update(&mut self, dt: f64) -> ScarabResult<()>;
}

pub trait UpdateChannel<T: Send> {
    // Call for updates that should happen on every game tick
    fn game_tick(&mut self, dt: f64) -> ScarabResult<()>;

    fn get_sender(&self) -> Sender<T>;

    fn consume_channel(&mut self) -> Option<Result<(), TryRecvError>>;

    fn update(&mut self, dt: f64) -> ScarabResult<()> {
        for _i in 0..CHANNEL_READS {
            if let Some(res) = self.consume_channel() {
                res?;
            } else {
                break;
            }
        }

        self.game_tick(dt);

        Ok(())
    }
}

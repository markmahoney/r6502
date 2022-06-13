use crossbeam::channel::{Receiver, tick};
use std::time::{Duration, Instant};

pub struct Clock {
    pub wait: Receiver<Instant>,
}

impl Clock {
    pub fn new(freq: f64) -> Self {
        Self {
            wait: tick(Duration::from_secs_f64(1.0 / freq)),
        }
    }
}

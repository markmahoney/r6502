use crossbeam::channel::{Receiver, tick};
use std::time::{Duration, Instant};

pub struct Clock {
    oscillator: Receiver<Instant>,
}

impl Clock {
    pub fn new(freq: f64) -> Self {
        Self {
            oscillator: tick(Duration::from_secs_f64(1.0 / freq)),
        }
    }

    pub fn start<F>(&self, mut callback: F)
    where F: FnMut() -> bool {
        for _ in self.oscillator.iter() {
            if callback() == false {
                break;
            }
        };
    }
}

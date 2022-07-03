use crossbeam::channel::{Receiver, Sender, bounded, tick};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

#[derive(Clone, Copy, Debug)]
pub enum LineState {
    LO,
    HI,
}

pub struct Clock {
    oscillator: Receiver<Instant>,
    phase1: LineState,
    phase2: LineState,
    phase1_queue: Vec<Sender<LineState>>,
    phase2_queue: Vec<Sender<LineState>>,
}

fn toggle(old: LineState) -> LineState {
    match old {
        LineState::LO => LineState::HI,
        LineState::HI => LineState::LO
    }
}

impl Clock {
    pub fn new(freq: f64) -> Self {
        Self {
            // Tick at twice the actual speed so we can toggle lo and hi at the correct(?) times
            oscillator: tick(Duration::from_secs_f64(1.0 / (freq * 2.0))),
            phase1: LineState::HI,
            phase2: LineState::LO,
            phase1_queue: vec![],
            phase2_queue: vec![],
        }
    }

    pub fn connect_phase1(&mut self) -> Receiver<LineState> {
        let (s, r) = bounded(0);
        self.phase1_queue.push(s);
        r
    }

    pub fn connect_phase2(&mut self) -> Receiver<LineState> {
        let (s, r) = bounded(1);
        self.phase2_queue.push(s);
        r
    }

    pub fn start(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let _ = self.oscillator.recv();
                self.update_state();
            };
        })
    }

    pub fn update_state(&mut self) {
        self.phase1 = toggle(self.phase1);
        self.phase2 = toggle(self.phase2);

        for s in self.phase1_queue.iter() {
            let _ = s.send(self.phase1);
        }

        for s in self.phase2_queue.iter() {
            let _ = s.send(self.phase2);
        }
    }
}

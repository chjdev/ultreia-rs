use crate::clock::{Clock, Tick, Tock};
use crate::observable::Observer;
use gdnative::prelude::*;
use std::sync::{Arc, Mutex, Weak};

struct ClockObserver {
    owner: Weak<Mutex<Node>>,
}

impl Observer<Tick> for ClockObserver {
    fn notify(&self, tick: &Tick) {
        self.owner.upgrade().iter().for_each(|owner| {
            owner.lock().expect("bla").emit_signal(
                GodotString::from_str("tick"),
                &[Variant::from_u64(tick.epoch() as u64)],
            );
        });
    }
}

impl Observer<Tock> for ClockObserver {
    fn notify(&self, tock: &Tock) {
        self.owner.upgrade().iter().for_each(|owner| {
            owner.lock().expect("bla").emit_signal(
                GodotString::from_str("tock"),
                &[Variant::from_u64(tock.epoch() as u64)],
            );
        });
    }
}

pub struct ClockEvents {
    observer: Arc<ClockObserver>,
}

impl ClockEvents {
    pub fn new(clock: &Clock, owner: &Arc<Mutex<Node>>) -> Self {
        let observer = Arc::new(ClockObserver {
            owner: Arc::downgrade(owner),
        });
        // clock.tickers().register(&observer);
        // clock.tockers().register(&observer);
        ClockEvents { observer }
    }
}

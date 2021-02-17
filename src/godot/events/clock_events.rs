use crate::clock::{Clock, Tick, Tock};
use crate::observable::Observer;
use gdnative::prelude::*;
use std::rc::Rc;

struct ClockObserver {
    owner: Ref<Node, Shared>,
}

impl Observer<Tick> for ClockObserver {
    fn notify(&self, tick: &Tick) {
        unsafe {
            self.owner.assume_safe().emit_signal(
                GodotString::from_str("tick"),
                &[Variant::from_u64(tick.epoch() as u64)],
            );
        }
    }
}

impl Observer<Tock> for ClockObserver {
    fn notify(&self, tock: &Tock) {
        unsafe {
            self.owner.assume_safe().emit_signal(
                GodotString::from_str("tock"),
                &[Variant::from_u64(tock.epoch() as u64)],
            );
        }
    }
}

pub struct ClockEvents {
    observer: Rc<ClockObserver>,
}

impl ClockEvents {
    pub fn new(clock: &Clock, owner: Ref<Node, Shared>) -> Self {
        let observer = Rc::new(ClockObserver { owner });
        clock.tickers().register(&observer);
        clock.tockers().register(&observer);
        ClockEvents { observer }
    }
}

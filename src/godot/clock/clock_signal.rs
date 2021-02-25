use crate::clock::{Clock, Tick, Tock};
use crate::observable::Observer;
use gdnative::prelude::*;
use std::sync::Arc;
use strum_macros::AsRefStr;

#[derive(Copy, Clone, PartialEq, Eq, AsRefStr)]
pub enum ClockSignal {
    Tick,
    Tock,
}

impl From<&Tick> for ClockSignal {
    fn from(_: &Tick) -> Self {
        ClockSignal::Tick
    }
}

impl From<&Tock> for ClockSignal {
    fn from(_: &Tock) -> Self {
        ClockSignal::Tock
    }
}

pub struct ClockObserver {
    owner: Ref<Node, Shared>,
}

impl Observer<Tick> for ClockObserver {
    fn notify(&self, tick: &Tick) {
        unsafe {
            self.owner.assume_safe().emit_signal(
                ClockSignal::from(tick),
                &[Variant::from_u64(tick.epoch() as u64)],
            );
        }
    }
}

impl Observer<Tock> for ClockObserver {
    fn notify(&self, tock: &Tock) {
        unsafe {
            self.owner.assume_safe().emit_signal(
                ClockSignal::from(tock),
                &[Variant::from_u64(tock.epoch() as u64)],
            );
        }
    }
}

impl ClockObserver {
    pub fn new(clock: &Clock, owner: Ref<Node, Shared>) -> Arc<Self> {
        let observer = Arc::new(ClockObserver { owner });
        clock.tickers().register(&observer);
        clock.tockers().register(&observer);
        observer
    }
}

use serde::{Serialize, Deserialize};
use crate::observable::{Observable, Observers};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Ord, Serialize, Deserialize)]
pub struct Tick(usize);

#[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Ord, Serialize, Deserialize)]
pub struct Tock(usize);

impl From<Tick> for Tock {
    fn from(tick: Tick) -> Self {
        Tock(tick.0)
    }
}

pub struct Clock {
    epoch: AtomicUsize,
    tickers: Observers<Tick>,
    tockers: Observers<Tock>,
}


impl Clock {
    pub fn new() -> Self {
        Clock {
            epoch: AtomicUsize::new(0),
            tickers: Observers::new(),
            tockers: Observers::new(),
        }
    }

    pub fn epoch(&self) -> usize {
        self.epoch.load(Ordering::Acquire)
    }

    pub fn tickers(&self) -> &Observers<Tick> {
        &self.tickers
    }

    pub fn tockers(&self) -> &Observers<Tock> {
        &self.tockers
    }

    pub fn tick(&self) {
        self.epoch.fetch_add(1, Ordering::Release);
        let tick = Tick(self.epoch());
        self.notify_all(&tick);
        self.tock(tick);
    }

    fn tock(&self, tick: Tick) {
        let tock = Tock::from(tick);
        self.notify_all(&tock);
    }
}

impl Observable<Tick> for Clock {
    fn observers(&self) -> &Observers<Tick> {
        &self.tickers
    }
}

impl Observable<Tock> for Clock {
    fn observers(&self) -> &Observers<Tock> {
        &self.tockers
    }
}

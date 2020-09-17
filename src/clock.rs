use crate::observable::{Observable, Observers};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Tick;

const TICK: Tick = Tick {};

pub struct Tock;

const TOCK: Tock = Tock {};

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
        self.notify_all(&TICK);
        self.tock();
    }

    fn tock(&self) {
        self.notify_all(&TOCK);
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

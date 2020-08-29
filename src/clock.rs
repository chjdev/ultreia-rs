use crate::observable::{Observable, Observers};

pub struct Tick;

const TICK: Tick = Tick {};

pub struct Tock;

const TOCK: Tock = Tock {};

pub struct Clock {
    tickers: Observers<Tick>,
    tockers: Observers<Tock>,
}


impl Clock {
    pub fn new() -> Self {
        Clock {
            tickers: Observers::new(),
            tockers: Observers::new(),
        }
    }

    pub fn tickers(&self) -> &Observers<Tick> {
        &self.tickers
    }

    pub fn tockers(&self) -> &Observers<Tock> {
        &self.tockers
    }

    pub fn tick(&self) {
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

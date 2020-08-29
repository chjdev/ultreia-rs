use crate::observable::{Observable, Observers};

pub struct Tick;

const TICK: Tick = Tick {};

pub struct Tock;

const TOCK: Tock = Tock {};

pub struct Clock<'a> {
    tickers: Observers<'a, Tick>,
    tockers: Observers<'a, Tock>,
}


impl<'a> Clock<'a> {
    pub fn new() -> Self {
        Clock {
            tickers: Observers::new(),
            tockers: Observers::new(),
        }
    }

    pub fn tickers(&'a self) -> &'a Observers<Tick> {
        &self.tickers
    }

    pub fn tockers(&'a self) -> &'a Observers<Tock> {
        &self.tockers
    }

    pub fn tick(&mut self) {
        self.notify_all(&TICK);
        self.tock();
    }

    fn tock(&mut self) {
        self.notify_all(&TOCK);
    }
}

impl<'a> Observable<'a, Tick> for Clock<'a> {
    fn observers(&self) -> &Observers<'a, Tick> {
        &self.tickers
    }
}

impl<'a> Observable<'a, Tock> for Clock<'a> {
    fn observers(&self) -> &Observers<'a, Tock> {
        &self.tockers
    }
}

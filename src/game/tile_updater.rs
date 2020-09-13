use crate::observable::{Observer, WeakObserver};
use crate::clock::{Tick, Clock};
use std::rc::{Weak, Rc};
use crate::map::Map;

struct TileUpdateObserver {
    map: Weak<Map>,
}

impl Observer<Tick> for TileUpdateObserver {
    fn notify(&self, _: &Tick) {
        if let Some(map) = self.map.upgrade() {
            if let Some(territory) = map.territories().get(0) {
                for (_, instance) in territory {
                    instance.update()
                }
            }
        }
    }
}

pub struct TileUpdater {
    clock: Weak<Clock>,
    observer: Rc<TileUpdateObserver>,
}

impl TileUpdater {
    pub fn new(clock: Weak<Clock>, map: Weak<Map>) -> Self {
        let observer = Rc::new(TileUpdateObserver { map });
        let weak_observer: WeakObserver<Tick> = Rc::downgrade(&observer) as WeakObserver<Tick>;
        clock.upgrade().map(|rc_clock| { rc_clock.tickers().register(weak_observer) });
        TileUpdater {
            observer,
            clock,
        }
    }
}

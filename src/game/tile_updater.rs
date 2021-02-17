use crate::clock::{Clock, Tick};
use crate::map::Map;
use crate::observable::Observer;
use std::rc::{Rc, Weak};

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
    observer: Rc<TileUpdateObserver>,
}

impl TileUpdater {
    pub fn new(clock: &Clock, map: &Rc<Map>) -> Self {
        let observer = Rc::new(TileUpdateObserver {
            map: Rc::downgrade(map),
        });
        clock.tickers().register(&observer);
        TileUpdater { observer }
    }
}

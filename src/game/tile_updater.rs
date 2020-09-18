use crate::observable::Observer;
use crate::clock::{Tick, Clock};
use std::sync::{Weak, Arc};
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
    observer: Arc<TileUpdateObserver>,
}

impl TileUpdater {
    pub fn new(clock: &Clock, map: &Arc<Map>) -> Self {
        let observer = Arc::new(TileUpdateObserver { map: Arc::downgrade(map) });
        clock.tickers().register(&observer);
        TileUpdater {
            observer,
        }
    }
}

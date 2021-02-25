use crate::clock::{Clock, Tick};
use crate::map::Map;
use crate::observable::Observer;
use std::sync::{Arc, RwLock, Weak};

pub struct TileUpdater {
    map: Weak<RwLock<Map>>,
}

impl Observer<Tick> for TileUpdater {
    fn notify(&self, _: &Tick) {
        // if let Some(map) = self.map.upgrade() {
        //     if let Some(territory) = map.territories().get(0) {
        //         for (_, instance) in territory {
        //             instance.update()
        //         }
        //     }
        // }
    }
}

impl TileUpdater {
    pub fn new(clock: &Clock, map: &Arc<RwLock<Map>>) -> Arc<Self> {
        let observer = Arc::new(TileUpdater {
            map: Arc::downgrade(map),
        });
        clock.tickers().register(&observer);
        observer
    }
}

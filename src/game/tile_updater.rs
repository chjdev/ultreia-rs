use crate::clock::{Clock, Tick, Tock};
use crate::coordinate::Coordinate;
use crate::map::buildings::SynchronizedInstance;
use crate::map::Map;
use crate::observable::Observer;
use crossbeam::utils::Backoff;
use rayon::prelude::*;
use std::sync::{Arc, RwLock, Weak};

pub struct TileUpdater {
    map: Weak<RwLock<Map>>,
}

impl Observer<Tick> for TileUpdater {
    fn notify(&self, _event: &Tick) {
        if let Some(map_arc) = self.map.upgrade() {
            let map = map_arc.read().unwrap();
            map.buildings.par_iter().for_each(
                |(coordinate, synchronized_instance): (&Coordinate, &SynchronizedInstance)| {
                    let mut instance = synchronized_instance.write().unwrap();
                    let influence = instance.tile().influence_at(coordinate);
                    for coordinate in influence {
                        let backoff = Backoff::new();
                        loop {
                            let maybe_other_instance_lock = map.buildings.try_mut(&coordinate);
                            if maybe_other_instance_lock.is_none() {
                                continue;
                            }
                            let other_instance_lock = maybe_other_instance_lock.unwrap();
                            if let Ok(mut other_instance) = other_instance_lock {
                                instance.consume(&mut *other_instance);
                                break;
                            }
                            backoff.spin()
                        }
                    }
                },
            );
        }
    }
}

impl Observer<Tock> for TileUpdater {
    fn notify(&self, _event: &Tock) {
        if let Some(map_arc) = self.map.upgrade() {
            let map = map_arc.read().unwrap();
            map.buildings.par_iter().for_each(
                |(_coordinate, synchronized_instance): (&Coordinate, &SynchronizedInstance)| {
                    let mut instance = synchronized_instance.write().unwrap();
                    instance.produce();
                },
            );
        }
    }
}

impl TileUpdater {
    pub fn new(clock: &Clock, map: Weak<RwLock<Map>>) -> Arc<Self> {
        let observer = Arc::new(TileUpdater { map });
        clock.tickers().register(&observer);
        clock.tockers().register(&observer);
        observer
    }
}

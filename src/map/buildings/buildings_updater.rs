use crate::clock::{Clock, Tick, Tock};
use crate::map::MapStorage;
use crate::observable::Observer;
use rayon::prelude::*;
use std::sync::{Arc, RwLock};

pub struct BuildingsUpdater {
    map_storage: Arc<RwLock<MapStorage>>,
}

impl Observer<Tick> for BuildingsUpdater {
    fn notify(&self, _event: &Tick) {
        let map = self.map_storage.read().unwrap();
        map.buildings.par_coordinates().for_each(|coordinate| {
            let mut mut_instance = map.buildings.spin_get_mut(coordinate);
            let influence = mut_instance.tile().influence_at(coordinate);
            for other_coordinate in influence {
                let mut other_mut_instance = map.buildings.spin_get_mut(&other_coordinate);
                mut_instance.consume(&mut *other_mut_instance);
            }
        });
    }
}

impl Observer<Tock> for BuildingsUpdater {
    fn notify(&self, _event: &Tock) {
        let map = self.map_storage.read().unwrap();
        map.buildings.par_coordinates().for_each(|coordinate| {
            let mut mut_instance = map.buildings.spin_get_mut(coordinate);
            mut_instance.produce()
        });
    }
}

impl BuildingsUpdater {
    pub fn new(clock: &Clock, map_storage: Arc<RwLock<MapStorage>>) -> Arc<Self> {
        let observer = Arc::new(BuildingsUpdater { map_storage });
        clock.tickers().register(&observer);
        clock.tockers().register(&observer);
        observer
    }
}

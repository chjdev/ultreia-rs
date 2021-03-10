use crate::clock::{Clock, Tick, Tock};
use crate::map::MapStorage;
use crate::observable::Observer;
use rayon::prelude::*;
use std::sync::{Arc, Weak};

pub struct BuildingsUpdater {
    map_storage: Weak<MapStorage>,
}

impl Observer<Tick> for BuildingsUpdater {
    fn notify(&self, _event: &Tick) {
        if let Some(map_storage_arc) = self.map_storage.upgrade() {
            map_storage_arc
                .buildings()
                .par_coordinates()
                .for_each(|coordinate| {
                    let buildings = map_storage_arc.buildings();
                    let mut mut_instance = buildings.spin_get_mut(coordinate);
                    let influence = mut_instance.tile().influence_at(coordinate);
                    for other_coordinate in influence {
                        let mut other_mut_instance = buildings.spin_get_mut(&other_coordinate);
                        mut_instance.consume(&mut *other_mut_instance);
                    }
                });
        }
    }
}

impl Observer<Tock> for BuildingsUpdater {
    fn notify(&self, _event: &Tock) {
        if let Some(map_storage_arc) = self.map_storage.upgrade() {
            map_storage_arc
                .buildings()
                .par_coordinates()
                .for_each(|coordinate| {
                    let buildings = map_storage_arc.buildings();
                    let mut mut_instance = buildings.spin_get_mut(coordinate);
                    mut_instance.produce()
                });
        }
    }
}

impl BuildingsUpdater {
    pub fn new(clock: &Clock, map: Weak<MapStorage>) -> Arc<Self> {
        let observer = Arc::new(BuildingsUpdater { map_storage: map });
        clock.tickers().register(&observer);
        clock.tockers().register(&observer);
        observer
    }
}

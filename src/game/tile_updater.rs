use crate::observable::{Observer, ObserverRegistration};
use crate::clock::{Tick, Clock};
use crate::map::tiles::TileMap;

struct TileUpdateObserver<'a> {
    tile_map: &'a mut TileMap,
}

impl<'a> Observer<Tick> for TileUpdateObserver<'a> {
    fn notify(&self, _: &Tick) {
        for (_, tile_instance) in self.tile_map.map().iter_mut() {
            tile_instance.update();
        }
    }
}

pub struct TileUpdater<'a> {
    clock: &'a Clock<'a>,
    registration: ObserverRegistration,
}

impl<'a> TileUpdater<'a> {
    pub fn new(clock: &'a Clock<'a>, tile_map: &'a mut TileMap) -> Self {
        TileUpdater {
            clock,
            registration: clock.tickers().register(Box::new(TileUpdateObserver { tile_map })),
        }
    }
}

impl<'a> Drop for TileUpdater<'a> {
    fn drop(&mut self) {
        self.clock.tickers().deregister(&self.registration);
    }
}



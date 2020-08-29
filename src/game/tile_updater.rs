use crate::observable::{Observer, ObserverRegistration};
use crate::clock::{Tick, Clock};
use std::rc::Weak;
use crate::map::Map;

struct TileUpdateObserver {
    map: Weak<Map>,
}

impl Observer<Tick> for TileUpdateObserver {
    fn notify(&self, _: &Tick) {
        if let Some(map) = self.map.upgrade() {
            for (_, tile_instance) in map.tiles().map().iter_mut() {
                tile_instance.update();
            }
        }
    }
}

pub struct TileUpdater<'a> {
    clock: Weak<Clock<'a>>,
    registration: Option<ObserverRegistration>,
}

impl<'a> TileUpdater<'a> {
    pub fn new(clock: Weak<Clock<'a>>, map: Weak<Map>) -> Self {
        TileUpdater {
            registration: clock.upgrade().map(|rc_clock| { rc_clock.tickers().register(Box::new(TileUpdateObserver { map })) }),
            clock,
        }
    }
}

impl<'a> Drop for TileUpdater<'a> {
    fn drop(&mut self) {
        if let Some(clock) = self.clock.upgrade() {
            if let Some(registration) = self.registration {
                clock.tickers().deregister(&registration);
            }
        }
    }
}



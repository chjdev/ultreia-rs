use crate::observable::Observer;
use crate::clock::{Tick, Clock};
use std::rc::{Weak, Rc};
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

pub struct TileUpdater {
    clock: Weak<Clock>,
    observer: Rc<dyn Observer<Tick>>,
}

impl TileUpdater {
    pub fn new(clock: Weak<Clock>, map: Weak<Map>) -> Self {
        let observer: Rc<dyn Observer<Tick>> = Rc::new(TileUpdateObserver { map });
        clock.upgrade().map(|rc_clock| { rc_clock.tickers().register(Rc::downgrade(&observer)) });
        TileUpdater {
            observer,
            clock,
        }
    }
}

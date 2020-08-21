use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::clock::{Clock, Tick};
use crate::map::tiles::{Map, TileMap};
use crate::observable::Observer;

pub struct TileUpdater {
    tile_map: Weak<RefCell<Map>>
}

impl TileUpdater {
    pub fn new(tile_map: Weak<RefCell<Map>>) -> Self {
        TileUpdater {
            tile_map
        }
    }
}

impl Observer<Tick> for TileUpdater {
    fn notify(&self, _: &Tick) {
        if let Some(mut tile_map) = self.tile_map.upgrade() {
            for (_, tile_instance) in tile_map.borrow_mut().iter_mut() {
                tile_instance.update();
            }
        }
    }
}

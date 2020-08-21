use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

use crate::coordinate::indexed::Indexed;
use crate::tile::TileInstance;

pub type Map = Indexed<Box<dyn TileInstance>>;

#[derive(Default)]
pub struct TileMap {
    map: Rc<RefCell<Map>>
}

impl TileMap {
    pub fn new() -> Self {
        TileMap {
            map: Rc::new(RefCell::new(Default::default()))
        }
    }

    pub fn map_weak(&self) -> Weak<RefCell<Map>> {
        Rc::downgrade(&self.map)
    }

    pub fn map(&self) -> RefMut<Map> {
        self.map.borrow_mut()
    }
}

// type MapStore<T> = HashMap<Coordinate, T>;

// pub struct Map<T> {
//     pub map: MapStore<T>,
// }
//
// impl<T> Map<T> {
//     pub fn new() -> Self {
//         Map {
//             map: Default::default(),
//         }
//     }
// }

// observers: Observers<MapEvent>,

// pub struct MapEvent;
// impl Observable<MapEvent> for Map {
//     fn observers(&mut self) -> &mut Observers<MapEvent> {
//         &mut self.observers
//     }
// }

use crate::coordinate::indexed::Indexed;
use crate::tile::TileInstance;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub type Map = Indexed<Box<dyn TileInstance>>;

pub struct Ground {
    map: Rc<RefCell<Map>>
}

impl Ground {
    pub fn new() -> Self {
        Ground {
            map: Rc::new(RefCell::new(Default::default()))
        }
    }

    pub fn map(&self) -> RefMut<Map> {
        self.map.borrow_mut()
    }
}

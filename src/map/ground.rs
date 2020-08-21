use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::coordinate::indexed::Indexed;
use crate::tile::TileInstance;

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

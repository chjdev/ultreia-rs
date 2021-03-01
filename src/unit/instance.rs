use crate::good::Inventory;
use crate::unit::{Unit, UnitInstance, UnitName};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct DefaultInstance {
    unit: UnitName,
    inventory: Option<RwLock<Inventory>>,
}

impl DefaultInstance {
    pub fn new(unit: &(impl Unit + ?Sized), inventory: Option<Inventory>) -> Arc<Self> {
        Arc::new(DefaultInstance {
            unit: *unit.name(),
            inventory: inventory.map(|s| RwLock::new(s)),
        })
    }
}

impl UnitInstance for DefaultInstance {
    fn unit(&self) -> &UnitName {
        &self.unit
    }

    fn inventory(&self) -> Option<RwLockReadGuard<Inventory>> {
        self.inventory.as_ref().map(|lock| lock.read().unwrap())
    }

    fn inventory_mut(&self) -> Option<RwLockWriteGuard<Inventory>> {
        self.inventory.as_ref().map(|lock| lock.write().unwrap())
    }

    fn move_by(&self, steps: usize) {
        unimplemented!()
    }

    fn movement_available(&self, steps: usize) -> usize {
        unimplemented!()
    }

    fn update(&self) {
        unimplemented!()
    }
}

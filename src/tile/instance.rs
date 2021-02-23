use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::{SomeTileInstance, State, Tile, TileInstance, TileName};
use std::sync::{LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct DefaultInstance {
    tile: TileName,
    state: Option<RwLock<State>>,
}

impl DefaultInstance {
    pub fn new(tile: TileName, state: Option<State>) -> Self {
        DefaultInstance {
            tile,
            state: state.map(|s| RwLock::new(s)),
        }
    }

    pub fn from(tile: &(impl Tile + ?Sized)) -> SomeTileInstance {
        Box::new(DefaultInstance::new(
            *tile.tile(),
            State::from(tile.consumes(), tile.produces()),
        ))
    }
}

impl TileInstance for DefaultInstance {
    fn tile(&self) -> &TileName {
        &self.tile
    }

    fn state(&self) -> Option<LockResult<RwLockReadGuard<'_, State>>> {
        self.state.as_ref().map(|rw_lock| rw_lock.read())
    }

    fn state_mut(&self) -> Option<LockResult<RwLockWriteGuard<'_, State>>> {
        self.state.as_ref().map(|rw_lock| rw_lock.write())
    }

    fn update(&self) {
        unimplemented!()
    }
}

impl std::ops::AddAssign<Inventory> for DefaultInstance {
    fn add_assign(&mut self, rhs: Inventory) {
        if let Some(s) = &self.state {
            let mut state = s.write().expect("could not lock state for reading");
            *state += &rhs;
        }
    }
}

impl std::ops::AddAssign<(Good, InventoryAmount)> for DefaultInstance {
    fn add_assign(&mut self, rhs: (Good, InventoryAmount)) {
        if let Some(s) = &self.state {
            let mut state = s.write().expect("could not lock state for reading");
            *state += (&rhs.0, &rhs.1);
        }
    }
}

// impl std::ops::AddAssign<&Inventory> for &DefaultInstance {
//     fn add_assign(&mut self, rhs: &Inventory) {
//         for tuple in rhs {
//             *self += tuple;
//         }
//     }
// }

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

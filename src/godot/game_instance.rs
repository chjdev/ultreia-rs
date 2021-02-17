use crate::game::{Configuration, Game};
use std::sync::{LockResult, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

lazy_static! {
    static ref INSTANCE: RwLock<Option<Game>> = RwLock::new(None);
}

pub struct GameController;

impl GameController {
    pub fn start(
        &mut self,
        configuration: Configuration,
    ) -> Result<(), PoisonError<RwLockWriteGuard<Option<Game>>>> {
        INSTANCE.write()?.replace(Game::new(configuration));
        Ok(())
    }

    pub fn game(&self) -> LockResult<RwLockReadGuard<Option<Game>>> {
        INSTANCE.read()
    }
}

use crate::game::{Configuration, Game};
use std::sync::{Arc, PoisonError, RwLock, RwLockWriteGuard};

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<Game>>> = RwLock::new(None);
}

pub struct GameController;

impl GameController {
    pub fn start(
        configuration: Configuration,
    ) -> Result<(), PoisonError<RwLockWriteGuard<'static, Option<Arc<Game>>>>> {
        INSTANCE
            .write()?
            .replace(Arc::new(Game::new(configuration)));
        Ok(())
    }

    pub fn game() -> Option<Arc<Game>> {
        INSTANCE.read().ok()?.as_ref().map(Arc::clone)
    }
}
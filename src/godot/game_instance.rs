use crate::game::{Configuration, Game};
use std::sync::RwLock;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Game>> = None;
}

pub struct GameController;

impl GameController {
    pub fn start(&mut self, configuration: Configuration) {
        INSTANCE
            .write()
            .expect("couldn't lock for write")
            .replace(Game::new(configuration));
    }

    pub fn game(&self) -> Option<&Game> {
        INSTANCE.read().expect("").as_ref()
    }
}

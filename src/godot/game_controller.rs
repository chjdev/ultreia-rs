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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinate::Coordinate;
    use crate::map::minimap::GetByCoordinate;
    use crate::map::terrain::{TerrainMeta, TerrainType};
    use strum::EnumCount;

    #[test]
    fn test_smoke() {
        GameController::start(Configuration::new(100, 100, 4.)).unwrap();
        let game = GameController::game().unwrap();
        let coordinate = Coordinate::default();
        let terrain_type: TerrainType = game.map().terrain().get(&coordinate);
        assert!((terrain_type as usize) < TerrainType::COUNT);
        let terrain_meta: TerrainMeta = game.map().terrain().get(&coordinate);
        assert!(terrain_meta.moisture() >= 0.);
    }
}

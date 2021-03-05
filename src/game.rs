use std::sync::{Arc, RwLock, RwLockReadGuard};

use buildings_controller::BuildingsController;

use crate::clock::Clock;
use crate::game::tile_updater::TileUpdater;
use crate::map::Map;

pub mod buildings_controller;
mod tile_updater;

#[derive(Copy, Clone)]
pub struct Configuration {
    rows: usize,
    columns: usize,
    island_noise: f64,
}

impl Configuration {
    pub fn new(rows: usize, columns: usize, island_noise: f64) -> Self {
        Configuration {
            rows,
            columns,
            island_noise,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn island_noise(&self) -> f64 {
        self.island_noise
    }
}

pub struct Game {
    configuration: Configuration,
    clock: Clock,
    map: Arc<RwLock<Map>>,
    buildings_controller: BuildingsController,
    tile_updater: Arc<TileUpdater>,
}

impl Game {
    pub fn new(configuration: Configuration) -> Self {
        let clock = Clock::new();
        let map = Arc::new(RwLock::new(Map::new(
            configuration.rows,
            configuration.columns,
            configuration.island_noise,
        )));
        Game {
            configuration,
            buildings_controller: BuildingsController::new(map.clone()),
            tile_updater: TileUpdater::new(&clock, Arc::downgrade(&map)),
            clock,
            map,
        }
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub fn map(&self) -> RwLockReadGuard<'_, Map> {
        self.map.read().unwrap()
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn buildings_controller(&self) -> &BuildingsController {
        &self.buildings_controller
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumCount;

    use crate::coordinate::Coordinate;
    use crate::map::minimap::GetByCoordinate;
    use crate::map::terrain::{TerrainMeta, TerrainType};

    use super::*;

    #[test]
    fn test_smoke() {
        let game = Game::new(Configuration::new(100, 100, 4.));
        let coordinate = Coordinate::default();
        let terrain_type: TerrainType = game.map().terrain().get(&coordinate);
        assert!((terrain_type as usize) < TerrainType::COUNT);
        let terrain_meta: TerrainMeta = game.map().terrain().get(&coordinate);
        assert!(terrain_meta.moisture() >= 0.);
    }
}

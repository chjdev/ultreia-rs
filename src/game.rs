mod tile_updater;

use crate::clock::Clock;
use crate::game::tile_updater::TileUpdater;
use crate::map::Map;
use std::sync::Arc;

#[derive(Copy, Clone)]
pub struct Configuration {
    rows: usize,
    columns: usize,
}

impl Configuration {
    pub fn new(rows: usize, columns: usize) -> Self {
        Configuration { rows, columns }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}

pub struct Game {
    configuration: Configuration,
    clock: Clock,
    map: Arc<Map>,
    tile_updater: TileUpdater,
}

impl Game {
    pub fn new(configuration: Configuration) -> Self {
        let clock = Clock::new();
        let map = Arc::new(Map::new(configuration.rows, configuration.columns));
        let tile_updater = TileUpdater::new(&clock, &map);
        Game {
            configuration,
            clock,
            map,
            tile_updater,
        }
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }
}

mod tile_updater;

use crate::clock::Clock;
use crate::game::tile_updater::TileUpdater;
use crate::map::Map;
use std::rc::Rc;

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
    map: Rc<Map>,
    tile_updater: TileUpdater,
}

impl Game {
    pub fn new(configuration: Configuration) -> Self {
        let clock = Clock::new();
        let map = Rc::new(Map::new(
            configuration.rows,
            configuration.columns,
            configuration.island_noise,
        ));
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

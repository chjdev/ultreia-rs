mod tile_updater;

use serde::{Serialize, Deserialize};
use crate::clock::Clock;
use crate::map::Map;
use crate::tile::TileFactory;
use crate::game::tile_updater::TileUpdater;
use std::rc::Rc;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Configuration {
    rows: usize,
    columns: usize,
}

impl Configuration {
    pub fn new(rows: usize, columns: usize) -> Self {
        Configuration {
            rows,
            columns,
        }
    }
}

pub struct Game {
    configuration: Configuration,
    clock: Rc<Clock>,
    map: Rc<Map>,
    tile_factory: Rc<TileFactory>,
    tile_updater: TileUpdater,
}

impl Game {
    pub fn new(configuration: Configuration) -> Self {
        let tile_factory = Rc::new(TileFactory::new());
        let clock = Rc::new(Clock::new());
        let map = Rc::new(Map::new(configuration.rows, configuration.columns, Rc::downgrade(&tile_factory)));
        let tile_updater = TileUpdater::new(Rc::downgrade(&clock), Rc::downgrade(&map));
        Game {
            configuration,
            clock,
            map,
            tile_factory,
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

    pub fn tile_factory(&self) -> &TileFactory {
        &self.tile_factory
    }
}

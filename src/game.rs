mod tile_updater;

use crate::clock::Clock;
use crate::map::Map;
use crate::tile::TileFactory;
use crate::game::tile_updater::TileUpdater;
use std::rc::Rc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Configuration {
    rows: usize,
    columns: usize,
}

pub struct Game {
    configuration: Configuration,
    clock: Rc<Clock>,
    map: Rc<Map>,
    tile_factory: TileFactory,
    tile_updater: TileUpdater,
}

impl Game {
    pub fn new(configuration: Configuration) -> Self {
        let clock = Rc::new(Clock::new());
        let map = Rc::new(Map::new(configuration.rows, configuration.columns));
        let tile_updater = TileUpdater::new(Rc::downgrade(&clock), Rc::downgrade(&map));
        Game {
            configuration,
            clock,
            map,
            tile_factory: TileFactory::new(),
            tile_updater,
        }
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

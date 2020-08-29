mod tile_updater;

use crate::clock::Clock;
use crate::map::Map;
use crate::tile::TileFactory;
use crate::game::tile_updater::TileUpdater;
use std::rc::Rc;

pub struct Game<'a> {
    clock: Rc<Clock<'a>>,
    map: Rc<Map>,
    tile_factory: TileFactory,
    tile_updater: TileUpdater<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let clock = Rc::new(Clock::new());
        let map = Rc::new(Map::new());
        let tile_updater = TileUpdater::new(Rc::downgrade(&clock), Rc::downgrade(&map));
        Game {
            clock,
            map,
            tile_factory: TileFactory::new(),
            tile_updater,
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn clock(&self) -> &Clock<'a> {
        &self.clock
    }

    pub fn tile_factory(&self) -> &TileFactory {
        &self.tile_factory
    }
}

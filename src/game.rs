use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::clock::Clock;
use crate::map::Map;
use crate::observable::Observable;
use crate::tile::TileFactory;
use crate::tile_updater::TileUpdater;

pub struct Game {
    clock: Clock,
    map: Map,
    tile_factory: TileFactory,
}

pub trait GameView<'a> {
    fn tile_factory(&self) -> &TileFactory;
    fn map(&self) -> &Map;
    fn clock(&'a self) -> &'a Clock;
}

impl Game {
    pub fn new() -> Self {
        Game {
            clock: Clock::new(),
            tile_factory: TileFactory::new(),
            map: Map::new(),
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

    pub fn start(&mut self) {
        // let map: &mut Map = &mut self.map;
        // let tile_updater: Box<TileUpdater> = Box::new(TileUpdater::new());
        // self.clock_mut().observers().register(tile_updater);
    }
}

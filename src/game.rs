mod tile_updater;

use crate::clock::Clock;
use crate::map::Map;
use crate::tile::TileFactory;
use crate::game::tile_updater::TileUpdater;

pub struct Session<'a> {
    tile_updater: TileUpdater<'a>
}

pub struct Game<'a, 'b: 'a> {
    clock: Clock<'b>,
    map: Map,
    tile_factory: TileFactory,
    session: Option<Session<'a>>,
}

impl<'a, 'b: 'a> Game<'a, 'b> {
    pub fn new() -> Self {
        let clock = Clock::new();
        let map = Map::new();
        Game {
            clock,
            map,
            tile_factory: TileFactory::new(),
            session: None,
        }
    }

    pub fn start(&'b mut self) -> Result<&Session<'a>, &'static str> {
        if self.session.is_some() {
            return Err("sesion already in progress");
        }
        let tile_updater = TileUpdater::new(&self.clock, self.map.tiles_mut());
        self.session = Some(Session {
            tile_updater
        });
        self.session.as_ref().ok_or("could not create session")
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn clock(&self) -> &Clock<'b> {
        &self.clock
    }

    pub fn clock_mut(&mut self) -> &mut Clock<'b> {
        &mut self.clock
    }

    pub fn tile_factory(&self) -> &TileFactory {
        &self.tile_factory
    }
}

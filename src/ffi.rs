use log::warn;
use crate::game::{Game, Configuration};
use crate::coordinate::Coordinate;
use crate::good::Good;
use crate::good::NaturalGood::CoalRepo;
use crate::map::terrain::TerrainTile;

trait MaybeMut {
    fn maybe_mut<F, R>(&self, fun: F) -> R where F: FnMut(&mut Game) -> R, R: Default;
}

impl MaybeMut for *mut Game {
    fn maybe_mut<F, R>(&self, fun: F) -> R where F: FnMut(&mut Game) -> R, R: Default {
        unsafe {
            self.as_mut().map(fun).unwrap_or(Default::default())
        }
    }
}

trait Maybe {
    fn maybe<F, R>(&self, fun: F) -> R where F: Fn(&Game) -> R, R: Default;
}

impl Maybe for *const Game {
    fn maybe<F, R>(&self, fun: F) -> R where F: Fn(&Game) -> R, R: Default {
        unsafe {
            self.as_ref().map(fun).unwrap_or(Default::default())
        }
    }
}

impl Maybe for *mut Game {
    fn maybe<F, R>(&self, fun: F) -> R where F: Fn(&Game) -> R, R: Default {
        (*self as *const Game).maybe(fun)
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_game(configuration: Configuration) -> *mut Game {
    Box::into_raw(Box::new(Game::new(configuration)))
}

#[no_mangle]
pub extern "C" fn drop_game(game_ptr: *mut Game) {
    if game_ptr.is_null() {
        warn!("no game at this pointer!");
        return;
    } else {
        unsafe {
            drop(Box::from_raw(game_ptr));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn clock_tick(game_ptr: *mut Game) {
    game_ptr.maybe_mut(|game| {
        game.clock().tick();
    });
}

#[no_mangle]
pub extern "C" fn map_terrain_get(game_ptr: *const Game, coordinate: Coordinate) -> TerrainTile {
    game_ptr.maybe(|game| {
        game.map().terrain().get(&coordinate)
    })
}

#[no_mangle]
pub extern "C" fn bla() -> Good {
    Good::NaturalGood(CoalRepo)
}

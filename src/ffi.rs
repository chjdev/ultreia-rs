use log::warn;
use crate::game::Game;
use crate::coordinate::Coordinate;
use crate::good::Good;
use crate::good::NaturalGood::CoalRepo;
use crate::map::terrain::TerrainTile;

trait MaybeMut {
    fn maybe_mut<F, R>(&self, fun: F) -> Option<R> where F: FnMut(&mut Game) -> R;
}

impl MaybeMut for *mut Game {
    fn maybe_mut<F, R>(&self, fun: F) -> Option<R> where F: FnMut(&mut Game) -> R {
        unsafe {
            self.as_mut().map(fun)
        }
    }
}

trait Maybe {
    fn maybe<F, R>(&self, fun: F) -> Option<R> where F: Fn(&Game) -> R;
}

impl Maybe for *const Game {
    fn maybe<F, R>(&self, fun: F) -> Option<R> where F: Fn(&Game) -> R {
        unsafe {
            self.as_ref().map(fun)
        }
    }
}

impl Maybe for *mut Game {
    fn maybe<F, R>(&self, fun: F) -> Option<R> where F: Fn(&Game) -> R {
        (*self as *const Game).maybe(fun)
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_game() -> *mut Game {
    Box::into_raw(Box::new(Game::new()))
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
    }).unwrap_or(TerrainTile::none())
}

#[no_mangle]
pub extern "C" fn bla() -> Good {
    Good::NaturalGood(CoalRepo)
}

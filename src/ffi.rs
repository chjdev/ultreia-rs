use log::warn;
use crate::game::Game;
use crate::coordinate::Coordinate;

trait MaybeMut {
    fn maybe_mut<F, R>(&self, fun: F) -> Option<R> where F: FnMut(&mut Game) -> R;
}

impl MaybeMut for *mut Game<'_, '_> {
    fn maybe_mut<F, R>(&self, fun: F) -> Option<R> where F: FnMut(&mut Game) -> R {
        unsafe {
            self.as_mut().map(fun)
        }
    }
}

trait Maybe {
    fn maybe<F, R>(&self, fun: F) -> Option<R> where F: Fn(&Game) -> R;
}

impl Maybe for *const Game<'_, '_> {
    fn maybe<F, R>(&self, fun: F) -> Option<R> where F: Fn(&Game) -> R {
        unsafe {
            self.as_ref().map(fun)
        }
    }
}

impl Maybe for *mut Game<'_, '_> {
    fn maybe<F, R>(&self, fun: F) -> Option<R> where F: Fn(&Game) -> R {
        (*self as *const Game<'_, '_>).maybe(fun)
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_game<'a, 'b: 'a>() -> *mut Game<'a, 'b> {
    let mut boxed: Box<Game<'a, 'b>> = Box::new(Game::new());
    boxed.start();
    let game_ptr: *mut Game<'a, 'b> = Box::into_raw(boxed);
    // game_ptr.maybe_mut(|game| { game.start() });
    game_ptr
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
        game.clock_mut().tick();
    });
}

#[no_mangle]
pub extern "C" fn map_get(game_ptr: *const Game, coordinate: Coordinate) -> i32 {
    game_ptr.maybe(|game| -> i32 {
        game.map().ground().map().get(&coordinate);
        1
    }).unwrap_or(-1)
}

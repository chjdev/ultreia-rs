use log::warn;
use crate::game::{Game, Configuration};
use crate::coordinate::Coordinate;
use crate::good::Good;
use crate::good::NaturalGood::CoalRepo;
use crate::map::terrain::{TerrainTile, TerrainType};

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

#[repr(C)]
pub struct Array<T> {
    pointer: *mut T,
    length: usize,
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(mut in_vec: Vec<T>) -> Self {
        in_vec.shrink_to_fit();
        let length = in_vec.len();
        assert_eq!(length, in_vec.capacity());
        let mut vec = in_vec.into_boxed_slice();
        let pointer = vec.as_mut_ptr();
        std::mem::forget(vec); // prevent deallocation in Rust
        // The array is still there but no Rust object
        // feels responsible. We only have ptr/len now
        // to reach it.
        Array {
            pointer,
            length,
        }
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Array {
            pointer: 0 as *mut T,
            length: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn map_terrain_rectangle(game_ptr: *const Game, from_corner: Coordinate, to_corner: Coordinate) -> Array<TerrainTile> {
    game_ptr.maybe(|game| {
        game.map().terrain().rectangle(&from_corner, &to_corner).into()
    })
}

#[no_mangle]
pub extern "C" fn free_terrain_tile_array(array: Array<TerrainTile>) {
    let s = unsafe { std::slice::from_raw_parts_mut(array.pointer, array.length as usize) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}

#[no_mangle]
pub extern "C" fn map_terrain_minimap(game_ptr: *const Game, width: u16, height: u16) -> Array<TerrainType> {
    game_ptr.maybe(|game| {
        game.map().terrain().minimap(width, height).into()
    })
}

#[no_mangle]
pub extern "C" fn free_terrain_type_array(array: Array<TerrainType>) {
    let s = unsafe { std::slice::from_raw_parts_mut(array.pointer, array.length as usize) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}


#[no_mangle]
pub extern "C" fn bla() -> Good {
    Good::NaturalGood(CoalRepo)
}

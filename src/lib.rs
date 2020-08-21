use std::ops::Deref;
use std::rc::Rc;

use crate::clock::Tick;
use crate::game::Game;
use crate::map::Map;
use crate::observable::Observable;
use crate::observable::Observer;
use crate::tile::Tiles;
use crate::tile_updater::TileUpdater;

mod coordinate;
mod game;
mod tile;
mod good;
mod map;
mod observable;
mod clock;
mod tile_updater;

pub fn hello() {
    // let mut game = Game::new();
    // let pio = game.tile_factory().create(Tiles::Pioneer);
    // let mut ground = game.map().ground().map();
    // ground.insert(Default::default(), pio);
    // ground.get(&Default::default()).map(|v| v.tile());
    // // ground.get_mut(&Default::default()).map(|v| v.update());
    // ground.get(&Default::default()).map(|v| v.tile());
    //
    // let tile_updater = Box::new(TileUpdater::new(game.map().tiles().map_weak()));
    // game.clock().observers().register(tile_updater);

    println!("Hello, world! 2");
}

#[no_mangle]
pub unsafe extern "C" fn create_game() -> *mut Game {
    Box::into_raw(Box::new(Game::new()))
}

#[no_mangle]
pub unsafe extern "C" fn bla(game: *mut Game) -> usize {
    game.as_ref().unwrap().map().ground().map().len()
    // 123
}


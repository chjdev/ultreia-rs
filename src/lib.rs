// todo remove
#![allow(dead_code)]

mod ffi;
pub use ffi::*;

mod coordinate;
mod game;
mod tile;
mod good;
mod map;
mod observable;
mod clock;
mod player;
// mod tile_updater;

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




// todo remove
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

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
mod server;

// todo remove
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod buildings_controller;
mod clock;
mod coordinate;
mod game;
mod godot;
mod good;
mod map;
mod observable;
mod saturating_from;
mod stacked_lru;
mod tile;
mod unit;
mod yields;

use serde::Deserialize;
use std::iter::FromIterator;
use actix_web::{web, HttpResponse, get, post};
use crate::coordinate::Coordinate;
use crate::game::Game;
use crate::tile::Tiles;
use crate::server::ok::ok;
use crate::server::with_coordinate::WithCoordinate;
use crate::coordinate::range::{RangeFrom, Range};


#[derive(Deserialize)]
pub struct CanContructRequest {
    x: i32,
    y: i32,
    z: i32,
}

#[get("/can_construct/{tile}")]
async fn map_can_construct(game: web::Data<Game>, web::Path(tile): web::Path<Tiles>, can_construct: web::Query<CanContructRequest>) -> HttpResponse {
    let coordinate = Coordinate::new(can_construct.x, can_construct.y, can_construct.z);
    ok(game.map().can_construct(&coordinate, tile))
}

#[derive(Deserialize)]
pub struct CanContructRectangleRequest {
    top_left: Coordinate,
    bottom_right: Coordinate,
}

#[post("/can_construct/{tile}/rectangle")]
async fn map_can_construct_rectangle(game: web::Data<Game>, web::Path(tile): web::Path<Tiles>, rectangle: web::Json<CanContructRectangleRequest>) -> HttpResponse {
    let range = rectangle.top_left.rectangle_to(&rectangle.bottom_right);
    ok(WithCoordinate::index(game.map().can_construct_range(&range, tile), &range))
}

#[get("/terrain")]
async fn map_terrain_get(game: web::Data<Game>, coordinate: web::Query<Coordinate>) -> HttpResponse {
    ok(game.map().terrain().get(&coordinate))
}

#[post("/terrain/range")]
async fn map_terrain_range(game: web::Data<Game>, range: web::Json<Vec<Coordinate>>) -> HttpResponse {
    let range = Range::from_iter(range.into_inner());
    ok(WithCoordinate::index(game.map().terrain().range(&range), &range))
}

#[derive(Deserialize)]
pub struct Size {
    width: u16,
    height: u16,
}

#[get("/terrain/minimap")]
async fn map_terrain_minimap(game: web::Data<Game>, size: web::Query<Size>) -> HttpResponse {
    ok(game.map().terrain().minimap(size.width, size.height))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/map")
            .service(map_can_construct)
            .service(map_can_construct_rectangle)
            .service(map_terrain_get)
            .service(map_terrain_range)
            .service(map_terrain_minimap)
    );
}
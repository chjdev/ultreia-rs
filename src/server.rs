mod with_coordinate;
mod game_pointer;

use std::iter::FromIterator;

use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, RecvError};
use actix_web::{get, post, web, App, HttpServer, HttpResponse};
use crate::game::{Configuration, Game};
use crate::coordinate::Coordinate;
use crate::server::with_coordinate::WithCoordinate;
use crate::coordinate::range::{Range, RangeFrom};
use crate::tile::Tiles;

fn ok<T: Serialize>(value: T) -> HttpResponse {
    HttpResponse::Ok().json(value)
}

#[get("/")]
async fn game_info(game: web::Data<Game>) -> HttpResponse {
    ok(*game.configuration())
}

#[get("/clock")]
async fn clock_epoch(game: web::Data<Game>) -> HttpResponse {
    ok(game.clock().epoch())
}

#[post("/clock")]
async fn clock_tick(game: web::Data<Game>) -> HttpResponse {
    game.clock().tick();
    ok(game.clock().epoch())
}

#[derive(Deserialize)]
pub struct CanContructRequest {
    x: i32,
    y: i32,
    z: i32,
}

#[get("/map/can_construct/{tile}")]
async fn map_can_construct(game: web::Data<Game>, web::Path(tile): web::Path<Tiles>, can_construct: web::Query<CanContructRequest>) -> HttpResponse {
    let coordinate = Coordinate::new(can_construct.x, can_construct.y, can_construct.z);
    ok(game.map().can_construct(&coordinate, tile))
}

#[derive(Deserialize)]
pub struct CanContructRectangleRequest {
    top_left: Coordinate,
    bottom_right: Coordinate,
}

#[post("/map/can_construct/{tile}/rectangle")]
async fn map_can_construct_rectangle(game: web::Data<Game>, web::Path(tile): web::Path<Tiles>, rectangle: web::Json<CanContructRectangleRequest>) -> HttpResponse {
    let range = rectangle.top_left.rectangle_to(&rectangle.bottom_right);
    ok(WithCoordinate::index(game.map().can_construct_range(&range, tile), &range))
}

#[get("/map/terrain")]
async fn map_terrain_get(game: web::Data<Game>, coordinate: web::Query<Coordinate>) -> HttpResponse {
    ok(game.map().terrain().get(&coordinate))
}

#[post("/map/terrain/range")]
async fn map_terrain_range(game: web::Data<Game>, range: web::Json<Vec<Coordinate>>) -> HttpResponse {
    let range = Range::from_iter(range.into_inner());
    ok(WithCoordinate::index(game.map().terrain().range(&range), &range))
}

#[derive(Deserialize)]
pub struct Size {
    width: u16,
    height: u16,
}

#[get("/map/terrain/minimap")]
async fn map_terrain_minimap(game: web::Data<Game>, size: web::Query<Size>) -> HttpResponse {
    ok(game.map().terrain().minimap(size.width, size.height))
}

#[actix_web::main]
async fn await_game_server(tx: Sender<u16>, configuration: Configuration) -> std::io::Result<()> {
    let server = HttpServer::new(move || App::new()
        .data(web::JsonConfig::default().limit(10000000)) // around 10mb
        .app_data(web::Data::new(Game::new(configuration)))
        .service(game_info)
        .service(clock_epoch)
        .service(clock_tick)
        .service(map_can_construct)
        .service(map_can_construct_rectangle)
        .service(map_terrain_get)
        .service(map_terrain_range)
        .service(map_terrain_minimap)
    ).bind("127.0.0.1:0")?;
    let random_port = server.addrs().get(0).unwrap().port();
    tx.send(random_port).unwrap();
    server.run().await
}

pub fn start_game_server(configuration: Configuration) -> Result<u16, RecvError> {
    let (tx, rx) = mpsc::channel::<u16>();
    std::thread::spawn(move || await_game_server(tx, configuration));
    rx.recv()
}
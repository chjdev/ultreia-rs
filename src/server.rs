mod with_coordinate;
mod ok;
mod scopes;

use std::sync::mpsc;
use std::sync::mpsc::{Sender, RecvError};
use actix_web::{get, web, App, HttpServer, HttpResponse};
use crate::game::{Configuration, Game};
use crate::server::ok::ok;

#[get("/")]
async fn game_info(game: web::Data<Game>) -> HttpResponse {
    ok(*game.configuration())
}

#[actix_web::main]
async fn await_game_server(tx: Sender<u16>, configuration: Configuration) -> std::io::Result<()> {
    let game = web::Data::new(Game::new(configuration));
    let server = HttpServer::new(move || App::new()
        .data(web::JsonConfig::default().limit(10000000)) // around 10mb
        .app_data(game.clone())
        .service(game_info)
        .configure(scopes::clock::config)
        .configure(scopes::map::config)
        .configure(scopes::events::config(game.as_ref()))
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
use actix_web::{web, HttpResponse, get, post};
use crate::game::Game;
use crate::server::ok::ok;

#[get("/clock")]
async fn clock_epoch(game: web::Data<Game>) -> HttpResponse {
    ok(game.clock().epoch())
}

#[post("/clock")]
async fn clock_tick(game: web::Data<Game>) -> HttpResponse {
    game.clock().tick();
    ok(game.clock().epoch())
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clock")
            .service(clock_epoch)
            .service(clock_tick)
    );
}
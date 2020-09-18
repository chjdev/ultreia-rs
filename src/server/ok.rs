use serde::Serialize;
use actix_web::HttpResponse;

pub fn ok<T: Serialize>(value: T) -> HttpResponse {
    HttpResponse::Ok().json(value)
}

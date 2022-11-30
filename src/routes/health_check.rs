use actix_web::{HttpResponse, get, Responder};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK!")
}
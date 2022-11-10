use actix_web::*;

pub async fn catalogo () -> HttpResponse {
    HttpResponse::Ok().body("Conectado...")
}
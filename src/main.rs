use actix_web::*;
//importando
mod routes;
use routes::ping::*;
use routes::info::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    let api = HttpServer::new(| | {
        App::new()
        .route("/", web::get().to(ping))
        .route("/info", web::get().to(info))
    });

    let porta = 9071;

    let api = api.bind(format!("127.0.0.1:{}", porta))
    .expect("Não consegui conectar...");

    println!("Conectado com sucesso! \n 🏒  http://localhost:{}/ping",porta );

    api.run()
    .await
}

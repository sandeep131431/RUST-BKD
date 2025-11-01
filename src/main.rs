mod modals;
mod db;
mod handler;
mod routes;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix-web server on http://localhost:8000");
    
    HttpServer::new(|| {
        let cors = Cors::permissive()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .configure(config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
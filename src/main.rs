mod modals;
mod db;
mod handler;
mod routes;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use routes::config;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Render ke liye PORT environment variable use karein
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");
    
    let host = "0.0.0.0"; // ✅ Important for cloud deployment
    
    println!("🚀 Starting Actix-web server on {}:{}", host, port);
    
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
    .bind((host, port))?  // ✅ Bind to 0.0.0.0 with dynamic port
    .run()
    .await
}
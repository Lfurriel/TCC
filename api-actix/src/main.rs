use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenvy::dotenv;
use std::env;
use crate::db::{AppState};

mod dal;
mod services;
pub mod schema;
pub mod db;
pub mod routes;
pub mod controllers;
pub mod utils;
pub mod configs;
pub mod middlewares;
pub mod models;
pub mod validations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = AppState::new();

    // Testar conexão na inicialização
    {
        let conn = app_state.db_pool.get()
            .expect("Failed to get DB connection on startup");
        log::info!("✅ Database connection successful");
        drop(conn);
    }

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{}:{}", host, port);

    log::info!("🚀 Starting server at http://{}", server_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .configure(routes::routes)
    })
        .bind(server_url)?
        .run()
        .await
}
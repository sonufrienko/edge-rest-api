use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::{env, io, sync::Mutex, time::Duration};

mod db_access;
mod handlers;
mod models;
mod routes;
mod state;

use routes::all_routes;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "warn,actix_web=info");
    env_logger::init();

    // Create a connection pool
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(100)
        .connect_timeout(std::time::Duration::new(30, 0))
        .connect(&db_url)
        .await
        .unwrap();

    // Shared data across threads
    let app_data = web::Data::new(AppState {
        health_check_response: String::from("OK"),
        visit_count: Mutex::new(0),
        db_pool,
    });

    // Construct App and routes
    let app = move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .configure(all_routes)
    };

    info!("Starting server on port 8080");

    // Start HTTP server
    HttpServer::new(app)
        .keep_alive(Duration::from_secs(60))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

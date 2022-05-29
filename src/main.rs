use actix_web::{middleware, web, App, HttpServer};
use std::{sync::Mutex, time::Duration};

mod handlers;
mod models;
mod routes;
mod state;

use routes::all_routes;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Shared data across threads
    let shared_data = web::Data::new(AppState {
        health_check_response: String::from("OK"),
        visit_count: Mutex::new(0),
    });

    // Construct App and routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .wrap(middleware::Logger::default())
            .configure(all_routes)
    };

    // Start HTTP server
    HttpServer::new(app)
        .keep_alive(Duration::from_secs(60))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

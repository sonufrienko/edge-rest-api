use actix_web::{middleware, App, HttpServer};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Construct App and routes
    let app = move || App::new().wrap(middleware::Logger::default());

    // Start HTTP server
    HttpServer::new(app)
        .keep_alive(Duration::from_secs(60))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

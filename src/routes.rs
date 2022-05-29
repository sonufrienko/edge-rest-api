use super::handlers::*;
use actix_web::{web, HttpResponse};

pub fn devices_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/devices")
            .route("", web::get().to(list_devices))
            .route("", web::post().to(create_device))
            .route("/{device_id}", web::post().to(get_device)),
    );
}

pub fn other_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(HttpResponse::Ok))
        .route("/ping", web::get().to(ping))
        .route("/health", web::get().to(health_check));
}

pub fn all_routes(cfg: &mut web::ServiceConfig) {
    devices_routes(cfg);
    other_routes(cfg);
}

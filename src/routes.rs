use super::handlers::{devices, general};
use actix_web::{web, HttpResponse};

pub fn devices_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/devices")
            .route("", web::get().to(devices::list))
            .route("", web::post().to(devices::create))
            .route("/{device_id}", web::get().to(devices::get)),
    );
}

pub fn other_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(HttpResponse::Ok))
        .route("/health", web::get().to(general::health_check));
}

pub fn all_routes(cfg: &mut web::ServiceConfig) {
    devices_routes(cfg);
    other_routes(cfg);
}

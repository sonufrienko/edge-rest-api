use super::models::*;
use super::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
// use log::{info, warn};

pub async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn create_device() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn list_devices() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn get_device(req: HttpRequest) -> HttpResponse {
    let device_id = req.match_info().get("device_id").unwrap_or("");
    dbg!(device_id);
    HttpResponse::Ok().finish()
}

pub async fn ping(req: HttpRequest) -> Result<impl Responder> {
    dbg!(&req.query_string());
    // HttpResponse::ok
    // web::Json(json!({ "temperature": 42.3 }))

    let id: i32 = req.match_info().query("id").parse().unwrap_or(9004);

    let obj = Ping {
        message: "Ping pong".to_string(),
        id,
    };

    Ok(web::Json(obj))
}

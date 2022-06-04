use crate::state::AppState;
use actix_web::{web, HttpResponse, Result};

pub async fn health_check(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    Ok(HttpResponse::Ok().json(response))
}

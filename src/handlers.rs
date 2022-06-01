use super::db_access::*;
use super::models::*;
use super::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use log::warn;

pub async fn health_check(app_state: web::Data<AppState>) -> Result<impl Responder> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    Ok(web::Json(response))
}

pub async fn create_device(
    app_state: web::Data<AppState>,
    body: web::Json<Device>,
) -> HttpResponse {
    match db_create_device(&app_state.db_pool, body.name.clone()).await {
        Err(e) => {
            warn!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(device) => HttpResponse::Ok().json(device),
    }
}

pub async fn list_devices(app_state: web::Data<AppState>) -> HttpResponse {
    match db_get_all_devices(&app_state.db_pool).await {
        Err(e) => {
            warn!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(devices) => HttpResponse::Ok().json(devices),
    }
}

pub async fn get_device(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let device_id = req.match_info().get("device_id").unwrap().to_owned();
    match db_get_device_by_id(&app_state.db_pool, device_id).await {
        Err(e) => {
            warn!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(device) => match device {
            Some(device) => HttpResponse::Ok().json(device),
            None => HttpResponse::NotFound().finish(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::test;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_web::test]
    async fn get_device_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_data = web::Data::new(AppState {
            health_check_response: String::from("OK"),
            visit_count: Mutex::new(0),
            db_pool,
        });

        let req = test::TestRequest::default()
            .param("device_id", "11-22".to_owned())
            .app_data(app_data.clone())
            .to_http_request();

        let resp = get_device(app_data, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

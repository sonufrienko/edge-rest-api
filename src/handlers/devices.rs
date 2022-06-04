use crate::dbaccess::devices::*;
use crate::errors::ApiError;
use crate::models::devices::*;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Result};

pub async fn create(
    app_state: web::Data<AppState>,
    body: web::Json<Device>,
) -> Result<HttpResponse, ApiError> {
    db_create_device(&app_state.db_pool, body.name.clone())
        .await
        .map(|data| HttpResponse::Ok().json(data))
}

pub async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    db_get_all_devices(&app_state.db_pool)
        .await
        .map(|data| HttpResponse::Ok().json(data))
}

pub async fn get(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let device_id = req.match_info().get("device_id").unwrap().to_owned();
    db_get_device_by_id(&app_state.db_pool, device_id)
        .await
        .map(|data| HttpResponse::Ok().json(data))
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

        let resp = get(app_data, req).await;
        assert!(resp.is_err());
    }

    #[actix_web::test]
    async fn list_devices_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_data = web::Data::new(AppState {
            health_check_response: String::from("OK"),
            visit_count: Mutex::new(0),
            db_pool,
        });

        let resp = list(app_data).await;
        assert!(resp.is_ok());
        assert_eq!(resp.unwrap().status(), StatusCode::OK);
    }
}

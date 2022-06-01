use super::models::*;
use super::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use chrono::Utc;
use log::debug;

pub async fn health_check(app_state: web::Data<AppState>) -> Result<impl Responder> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    Ok(web::Json(response))
}

pub async fn create_device(body: web::Json<Device>) -> HttpResponse {
    let device = Device {
        device_id: body.device_id.clone(),
        name: body.name.clone(),
        registered_at: Some(Utc::now().naive_utc()),
    };

    debug!("New device created: {:?}", &device);
    HttpResponse::Ok().json(device)
}

pub async fn list_devices() -> HttpResponse {
    let devices: Vec<Device> = vec![
        Device {
            device_id: String::from("11-11"),
            name: String::from("Arduino UNI"),
            registered_at: Some(Utc::now().naive_utc()),
        },
        Device {
            device_id: String::from("11-22"),
            name: String::from("Arduino MKR"),
            registered_at: Some(Utc::now().naive_utc()),
        },
    ];

    HttpResponse::Ok().json(devices)
}

pub async fn get_device(req: HttpRequest) -> HttpResponse {
    let device_id = req.match_info().get("device_id").unwrap().to_owned();

    dbg!(&device_id);

    if device_id.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    let device = Device {
        device_id,
        name: String::from("Ardiono UNO"),
        registered_at: Some(Utc::now().naive_utc()),
    };

    HttpResponse::Ok().json(device)
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
            db: db_pool,
        });

        let req = test::TestRequest::default()
            .param("device_id", "11-22".to_owned())
            .app_data(app_data)
            .to_http_request();

        let resp = get_device(req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

use super::errors::ApiError;
use super::models::*;
use sqlx::types::uuid::Uuid;
use sqlx::{query_as, PgPool};

pub async fn db_get_all_devices(db_pool: &PgPool) -> Result<Vec<Device>, ApiError> {
    let rows = query_as!(
        Device,
        r#"SELECT device_id, name, registered_at FROM public.devices ORDER BY registered_at DESC"#
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}

pub async fn db_get_device_by_id(
    db_pool: &PgPool,
    device_id: String,
) -> Result<Option<Device>, ApiError> {
    // Validate UUID
    Uuid::parse_str(&device_id)?;

    let row = query_as!(
        Device,
        r#"SELECT device_id, name, registered_at FROM public.devices WHERE device_id = $1"#,
        device_id,
    )
    .fetch_optional(db_pool)
    .await?;

    match row {
        None => Err(ApiError::NotFound("Device not found".into())),
        _ => Ok(row),
    }
}

pub async fn db_create_device(db_pool: &PgPool, name: String) -> Result<Device, ApiError> {
    if name.is_empty() {
        return Err(ApiError::RequestError("Device name is required".into()));
    }

    let row = query_as!(
        Device,
        r"INSERT INTO public.devices (name) VALUES ($1) RETURNING device_id, name, registered_at",
        name,
    )
    .fetch_one(db_pool)
    .await?;

    Ok(row)
}

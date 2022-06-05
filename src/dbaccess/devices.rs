use crate::errors::ApiError;
use crate::models::devices::{CreateDevice, Device, UpdateDevice};
use sqlx::types::uuid::Uuid;
use sqlx::{query_as, PgPool};

pub async fn db_get_all_devices(db_pool: &PgPool) -> Result<Vec<Device>, ApiError> {
    let rows = query_as!(
        Device,
        r#"SELECT device_id, name, registered_at, status FROM public.devices ORDER BY registered_at DESC"#
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
        r#"SELECT device_id, name, registered_at, status FROM public.devices WHERE device_id = $1"#,
        device_id,
    )
    .fetch_optional(db_pool)
    .await?;

    match row {
        None => Err(ApiError::NotFound("Device not found".into())),
        _ => Ok(row),
    }
}

pub async fn db_create_device(db_pool: &PgPool, device: CreateDevice) -> Result<Device, ApiError> {
    if device.name.is_empty() {
        return Err(ApiError::RequestError("Device name is required".into()));
    }

    let row = query_as!(
        Device,
        r"INSERT INTO public.devices (name, status) VALUES ($1, $2) RETURNING device_id, name, registered_at, status",
        device.name,
        device.status,
    )
    .fetch_one(db_pool)
    .await?;

    Ok(row)
}

pub async fn db_update_device(
    db_pool: &PgPool,
    device_id: String,
    device: UpdateDevice,
) -> Result<Option<Device>, ApiError> {
    if device.name.is_empty() {
        return Err(ApiError::RequestError("Device name is required".into()));
    }

    let row = query_as!(
        Device,
        r"UPDATE public.devices 
        SET name = $1, status = $2
        WHERE device_id = $3
        RETURNING device_id, name, registered_at, status",
        device.name,
        device.status,
        device_id,
    )
    .fetch_optional(db_pool)
    .await?;

    match row {
        None => Err(ApiError::NotFound("Device not found".into())),
        _ => Ok(row),
    }
}

pub async fn db_delete_device(db_pool: &PgPool, device_id: String) -> Result<bool, ApiError> {
    let course_row = sqlx::query!("DELETE FROM public.devices where device_id = $1", device_id,)
        .execute(db_pool)
        .await?;

    Ok(course_row.rows_affected() > 0)
}

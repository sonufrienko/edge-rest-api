use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Device {
    pub device_id: String,
    pub name: String,
    pub registered_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceData {
    pub id: String,
    pub data: String,
    pub posted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceConfiguration {
    pub version: u32,
    pub changed_at: Option<NaiveDateTime>,
}
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Device {
    pub device_id: String,
    pub name: String,
    pub registered_at: NaiveDateTime,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateDevice {
    pub name: String,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateDevice {
    pub name: String,
    pub status: Option<String>,
}

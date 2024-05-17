use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceConfig {
    pub name: String,
    pub description: String,
    pub host: String,
    pub port: i16,
    pub secure: Option<bool>,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub interval: Option<i32>,
    pub retry_interval: Option<i32>,
    pub interval_timeout: Option<i32>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        ServiceConfig {
            name: String::new(),
            description: String::new(),
            host: String::new(),
            port: 80,
            secure: None,
            user: None,
            pass: None,
            interval: Some(300),
            retry_interval: Some(0),
            interval_timeout: Some(48),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    service_id: i32,
    status: ServiceStatus,
    time: SystemTime,
    msg: String,
    duration: Duration,
    retries: i16,
}

#[derive(Deserialize, Debug)]
pub enum ServiceStatus {
    Up,
    Down,
    Pending,
}

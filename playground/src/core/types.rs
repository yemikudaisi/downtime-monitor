use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceConfig {
    pub id: Option<i64>,
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
            id: None,
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
    service_id: i64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config_defaults() {
        let cfg = ServiceConfig {
            id: None,
            name: "Service 2".to_string(),
            description: "My service 2".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            ..Default::default()
        };
        assert_eq!(cfg.interval.unwrap(), 300)
    }
}

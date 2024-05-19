use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;

///
/// Contains service client configuration used for verification
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
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

///
/// Default implementation of `ServiceConfig` struct.`
impl Default for ServiceConfig {
    fn default() -> Self {
        ServiceConfig {
            id: None,
            name: String::new(),
            description: String::new(),
            host: String::new(),
            port: 80,
            secure: Some(false),
            user: Some("".to_string()),
            pass: Some("".to_string()),
            interval: Some(300),
            retry_interval: Some(0),
            interval_timeout: Some(48),
            created_at: Some(Utc::now().to_rfc3339()),
            updated_at: Some(Utc::now().to_rfc3339()),
        }
    }
}

///
/// Contains data relating to each service configuration
///
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Heartbeat {
    service_id: i64,
    status: ServiceStatus,
    time: SystemTime,
    msg: String,
    duration: Duration,
    retries: i16,
}

///
/// Represents the status of services
#[derive(Deserialize, Debug)]
pub enum ServiceStatus {
    Up,
    Down,
    Pending,
}

// mod iso8601 {
//     use chrono::{DateTime, Utc};
//     use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

//     const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

//     pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         date.map(|d| d.format(FORMAT).to_string())
//             .serialize(serializer)
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s: Option<String> = Option::deserialize(deserializer)?;
//         Ok(s.and_then(|s| DateTime::parse_from_str(&s, FORMAT).ok()))
//     }
// }

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

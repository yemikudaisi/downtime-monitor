use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

///
/// Contains service client configuration used for verification
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceParameters {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub host: String,
    pub port: u16,
    pub secure: Option<bool>,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub interval: Option<u32>,
    pub retry_interval: Option<u32>,
    pub interval_timeout: Option<u32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

///
/// Default implementation of `ServiceConfig` struct.`
impl Default for ServiceParameters {
    fn default() -> Self {
        ServiceParameters {
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
    pub id: Option<i64>,
    pub service_id: i64,
    pub status: ServiceStatus,
    pub time: String,
    pub msg: String,
    pub duration: u64,
    pub retries: i16,
}

///
/// Result of serviec verification. Determines if its up.
/// Contains error meesage if the service is down.
#[derive(Serialize, Debug, Clone)]
pub struct ServiceVerificationResult {
    pub success: bool,
    pub message: String,
}

impl Display for ServiceVerificationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Succes: {}, message: {}", self.success, self.message)
    }
}

///
/// Represents the status of services
#[derive(Deserialize, Debug)]
pub enum ServiceStatus {
    Up,
    Down,
    Pending,
}

impl FromStr for ServiceStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ip" {
            return Ok(ServiceStatus::Up);
        }
        if s == "down" {
            return Ok(ServiceStatus::Down);
        }
        if s == "pending" {
            return Ok(ServiceStatus::Pending);
        }
        Ok(ServiceStatus::Down)
    }
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServiceStatus::Up => write!(f, "up"),
            ServiceStatus::Down => write!(f, "down"),
            ServiceStatus::Pending => write!(f, "pending"),
        }
    }
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
        let cfg = ServiceParameters {
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

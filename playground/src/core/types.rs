use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

const HTTP_SERVICE_CODE: &str = "http";
const SMTP_SERVICE_CODE: &str = "smtp";
const FTP_SERVICE_CODE: &str = "ftp";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceType {
    Http,
    Smtp,
    Ftp,
}

impl ServiceType {
    pub fn to_str(&self) -> &'static str {
        match self {
            ServiceType::Http => HTTP_SERVICE_CODE,
            ServiceType::Smtp => SMTP_SERVICE_CODE,
            ServiceType::Ftp => FTP_SERVICE_CODE,
        }
    }

    pub fn from_string(service_str: &str) -> ServiceType {
        match service_str {
            HTTP_SERVICE_CODE => ServiceType::Http,
            SMTP_SERVICE_CODE => ServiceType::Smtp,
            FTP_SERVICE_CODE => ServiceType::Ftp,
            _ => ServiceType::Http,
        }
    }
}

impl std::fmt::Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::Http => write!(f, "{}", HTTP_SERVICE_CODE.to_string()),
            ServiceType::Smtp => write!(f, "{}", SMTP_SERVICE_CODE.to_string()),
            ServiceType::Ftp => write!(f, "{}", FTP_SERVICE_CODE.to_string()),
        }
    }
}

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
    pub service_type: ServiceType,
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
            service_type: ServiceType::Http,
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
    pub service_id: i64,
    pub success: bool,
    pub message: String,
}

impl Display for ServiceVerificationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Service [{}] verification result -> [success: {}, message: {}]",
            self.service_id, self.success, self.message
        )
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

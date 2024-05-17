use lettre::transport::smtp::authentication::Credentials;
use lettre::{transport::smtp::SmtpTransportBuilder, SmtpTransport};
use reqwest::header::HeaderValue;
use reqwest::Client;
use reqwest::StatusCode;
use serde::Serialize;

use super::types::ServiceConfig;

#[derive(Serialize, Debug)]
pub struct ServiceVerificationResult {
    pub success: bool,
    pub message: String,
}

/// .
/// Verifies a SMTP service is up
/// ## Panics
///
/// Panics if .
///
/// ## Errors
///
/// This function will return an error.
pub fn verify_smtp(smtp_config: ServiceConfig) -> ServiceVerificationResult {
    let mut builder: SmtpTransportBuilder;
    match (smtp_config.secure != None && smtp_config.secure.unwrap()) {
        true => {
            let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
                SmtpTransport::starttls_relay(&smtp_config.host);
            match result {
                Ok(smtp_transport_builder) => builder = smtp_transport_builder,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return ServiceVerificationResult {
                        success: false,
                        message: format!("{}", e),
                    };
                }
            }
        }
        false => {
            builder = SmtpTransport::builder_dangerous(&smtp_config.host);
            println!("[+] Testing SMTP without TLS.");
        }
    };
    builder = builder.port(smtp_config.port as u16);

    let transport: SmtpTransport;

    if !(smtp_config.user.as_ref() == None) {
        let creds = Credentials::new(smtp_config.user.unwrap(), smtp_config.pass.unwrap().clone());
        transport = builder.credentials(creds).build();
    } else {
        transport = builder.build();
    }

    match transport.test_connection() {
        Ok(_) => ServiceVerificationResult {
            success: true,
            message: "Connection successful".to_string(),
        },
        Err(e) => ServiceVerificationResult {
            success: false,
            message: format!("Test failed: {}", e),
        },
    }
}

pub async fn verify_website(website_config: ServiceConfig) -> ServiceVerificationResult {
    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("hello"));
    let full_url;
    if website_config.port != 80 {
        full_url = format!("{}:{}", website_config.host, website_config.port);
    } else {
        full_url = website_config.host;
    }
    // Send a GET request to the specified URL
    match client.get(&full_url).headers(headers).send().await {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                ServiceVerificationResult {
                    success: true,
                    message: format!("Website {} is online", full_url),
                }
            } else {
                ServiceVerificationResult {
                    success: false,
                    message: format!(
                        "Website {} returned status code: {}",
                        full_url,
                        response.status()
                    ),
                }
            }
        }
        Err(e) => ServiceVerificationResult {
            success: false,
            message: format!("Error: {}", e),
        },
    }
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lettre::transport::smtp::authentication::Credentials;
use lettre::{transport::smtp::SmtpTransportBuilder, SmtpTransport};
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header::{HeaderValue, InvalidHeaderValue};
use serde::{Deserialize, Serialize};
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let smtp_config = ServiceConfig {
        host: String::from("smtp.freesmtpservers.com"),
        port: 25,
        secure: false,
        user: String::from(""),
        pass: String::from(""),
    };
    let result = verify_smtp(smtp_config);
    match result.success {
        true => {
            println!("Success: {}", result.message)
        }
        false => {
            println!("Failed: {}", result.message)
        }
    }

    let website_config = ServiceConfig {
        host: String::from("https://www.army.mil.ng"),
        port: 80,
        secure: false,
        user: String::from(""),
        pass: String::from(""),
    };

    let result = verify_website(website_config).await;
    match result.success {
        true => {
            println!("Website > Success: {}", result.message)
        }
        false => {
            println!("Website > Failed: {}", result.message)
        }
    }
}

#[derive(Deserialize, Debug)]
struct ServiceConfig {
    host: String,
    port: i16,
    secure: bool,
    user: String,
    pass: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServiceVerificationResult {
    success: bool,
    message: String,
}

fn verify_smtp(smtp_config: ServiceConfig) -> ServiceVerificationResult {
    let creds = Credentials::new(smtp_config.user.clone(), smtp_config.pass.clone());

    let mut builder: SmtpTransportBuilder;
    if smtp_config.secure {
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
    } else {
        builder = SmtpTransport::builder_dangerous(&smtp_config.host);
        println!("[+] Testing SMTP without TLS.");
    };
    builder = builder.port(smtp_config.port as u16);

    let transport: SmtpTransport;

    if !(smtp_config.user.is_empty()) {
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

async fn verify_website(website_config: ServiceConfig) -> ServiceVerificationResult {
    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("hello"));
    let full_url;
    if website_config.port != 80 {
      full_url= format!("{}:{}", website_config.host, website_config.port);
    }else{
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

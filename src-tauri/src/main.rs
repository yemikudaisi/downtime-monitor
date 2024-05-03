// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//use std::hint;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{transport::smtp::SmtpTransportBuilder, SmtpTransport};
// use lettre::transport::smtp::client::SmtpConnection;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use tauri::Manager;
use window_vibrancy::apply_acrylic;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = apply_acrylic(&window, Some((0, 0, 0, 10)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![verify_smtp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize, Debug)]
struct SmtpConfig {
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

fn concat_str_format<'a>(a: &'a str, b: &'a str) -> Cow<'a, str> {
    format!("{}{}", a, b).into()
}

#[tauri::command]
fn verify_smtp(smtp_config: SmtpConfig) -> ServiceVerificationResult {
    let failure_result = ServiceVerificationResult {
        success: false,
        message: "Connection failed".to_string(),
    };

    print!("host -> {}", smtp_config.host);

    let host = smtp_config.host.as_str();
    let port_string = smtp_config.port.to_string();
    let port = port_string.as_str();

    let mut builder: SmtpTransportBuilder;
    if smtp_config.secure {
        let relay = concat_str_format(host, port);
        let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
            SmtpTransport::starttls_relay(&relay);
        match result {
            Ok(smtp_transport_builder) => builder = smtp_transport_builder,
            Err(e) => {
                return failure_result;
            }
        }
    } else {
        let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
            SmtpTransport::relay(&smtp_config.host);
        match result {
            Ok(smtp_transport_builder) => builder = smtp_transport_builder,
            Err(e) => {
                return failure_result;
            }
        }
    };

    // Set credentials (if needed)
    if !smtp_config.user.is_empty() && !smtp_config.pass.is_empty() {
        let credentials = Credentials::new(smtp_config.user.clone(), smtp_config.pass.clone());
        builder = builder.credentials(credentials);
    }

    let transport = builder.build();
    match transport.test_connection() {
        Ok(_) => {
            return ServiceVerificationResult {
                success: true,
                message: "This is success".to_string(),
            }
        }
        Err(_) => return failure_result,
    }
}

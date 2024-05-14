// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//use std::hint;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{transport::smtp::SmtpTransportBuilder, SmtpTransport};

use reqwest::blocking::Client;
use reqwest::StatusCode;

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

#[tauri::command]
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

#[tauri::command]
fn verify_website(website_config: ServiceConfig) -> ServiceVerificationResult {
  let client = Client::new();
  let full_url = format!("{}:{}", website_config.host, website_config.port);
  // Send a GET request to the specified URL
  match client.get(&full_url).send() {
      Ok(response) => {
          if response.status() == StatusCode::OK {
              ServiceVerificationResult {
                  success: true,
                  message: format!("Website {} is online", full_url),
              }
          } else {
              ServiceVerificationResult {
                  success: false,
                  message: format!("Website {} returned status code: {}", full_url, response.status()),
              }
          }
      }
      Err(e) => ServiceVerificationResult {
          success: false,
          message: format!("Error: {}", e),
      },
  }
}
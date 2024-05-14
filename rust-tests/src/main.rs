// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lettre::transport::smtp::authentication::Credentials;
use lettre::{transport::smtp::SmtpTransportBuilder, SmtpTransport};
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
    let config = SmtpConfig {
        host: String::from("smtp.freesmtpservers.com"),
        port: 25,
        secure: false,
        user: String::from(""),
        pass: String::from(""),
    };
    let result = verify_smtp(config);
    match result.success {
        true => {
            println!("Success: {}", result.message)
        }
        false => {
            println!("Failed: {}", result.message)
        }
    }
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

fn verify_smtp_unsecure(smtp_config: SmtpConfig) -> ServiceVerificationResult {
    let builder = SmtpTransport::builder_dangerous(&smtp_config.host);
    println!("[+] Testing SMTP without TLS.");
    // match result {
    //     Ok(smtp_transport_builder) => builder = smtp_transport_builder,
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //         return ServiceVerificationResult {
    //             success: false,
    //             message: format!("{}", e),
    //         };
    //     }
    // }

    let builder = builder.port(smtp_config.port as u16);
    let transport = builder.build();

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

fn verify_smtp(smtp_config: SmtpConfig) -> ServiceVerificationResult {
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
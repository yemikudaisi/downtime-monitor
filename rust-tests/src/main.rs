// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//use std::hint;

// use lettre::transport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{transport::smtp::SmtpTransportBuilder, SmtpTransport};
// use lettre::transport::smtp::client::SmtpConnection;
use serde::{Deserialize, Serialize};
// use std::borrow::Cow;


fn main() {
    println!("Hello, world!");
    let config = SmtpConfig{
        host: String::from("smtp.freesmtpservers.com"),
        port: 25,
        secure: false,
        user: String::from(""),
        pass: String::from(""),
    };
    let result = verify_smtp(config);
    match result.success {
        true => {println!("Success: {}", result.message)},
        false => {println!("Failed: {}", result.message)}
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

fn verify_smtp(smtp_config: SmtpConfig) -> ServiceVerificationResult {
    let creds = Credentials::new(smtp_config.user.clone(), smtp_config.pass.clone());

    let builder: SmtpTransportBuilder;
    if smtp_config.secure {
        let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
            SmtpTransport::starttls_relay(&smtp_config.host);
        match result {
            Ok(smtp_transport_builder) => builder = smtp_transport_builder,
            Err(e) => {
                println!("Error: {:?}", e);
                return ServiceVerificationResult { success: false, message: e.to_string()};
            }
        }
    } else {
        let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
            SmtpTransport::relay(&smtp_config.host);
        match result {
            Ok(smtp_transport_builder) => builder = smtp_transport_builder,
            Err(e) => {
                println!("Error: {:?}", e);
                return ServiceVerificationResult { success: false, message: e.to_string()};
            }
        }
    };
    let builder = builder.port(smtp_config.port as u16);

    let transport: SmtpTransport;

    if !(smtp_config.user.is_empty()) {
        transport = builder.credentials(creds).build();
    } else{
        transport = builder.build();
    }

    match transport.test_connection() {
        Ok(_) => ServiceVerificationResult {
            success: true,
            message: "Connection successful".to_string(),
        },
        Err(_) => ServiceVerificationResult {
            success: false,
            message: "Connection failed".to_string(),
        },
    }
}



// fn concat_str_format<'a>(a: &'a str, b: &'a str) -> Cow<'a, str> {
//     format!("{}:{}", a, b).into()
// }

// // fn concat_str_format<'a>(a: &'a str, b: &'a str) -> Cow<'a, str> {
// //     format!("{}:{}", a, b).into()
// // }

// fn verify_smtp(smtp_config: SmtpConfig) -> ServiceVerificationResult {
//     let failure_result = ServiceVerificationResult {
//         success: false,
//         message: "Connection failed".to_string(),
//     };

//     let host = smtp_config.host.as_str();
//     let port_string = smtp_config.port.to_string();
//     let port = port_string.as_str();

//     let mut builder: SmtpTransportBuilder;    
//     let relay = concat_str_format(host, port);    
//     println!("Relay -> {}", relay);
//     if smtp_config.secure {
//         let creds = Credentials::new(smtp_config.user, smtp_config.pass);
//         let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
//             SmtpTransport::starttls_relay(&relay);
//         match result {
//             Ok(smtp_transport_builder) => builder = smtp_transport_builder,
//             Err(e) => {
//                 println!("Error: {:?}", e);
//                 return failure_result;
//             }
//         }
//     } else {
//         let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
//             SmtpTransport::relay(&smtp_config.host);
//         match result {
//             Ok(smtp_transport_builder) => builder = smtp_transport_builder,
//             Err(e) => {
//                 println!("Error: {:?}", e);
//                 return failure_result;
//             }
//         }
//     };

//     // Set credentials (if needed)
//     if !smtp_config.user.is_empty() && !smtp_config.pass.is_empty() {
//         let credentials = Credentials::new(smtp_config.user.clone(), smtp_config.pass.clone());
//         builder = builder.credentials(credentials);
//     }

//     let transport = builder.build();
//     match transport.test_connection() {
//         Ok(_) => {
//             return ServiceVerificationResult {
//                 success: true,
//                 message: "This is success".to_string(),
//             }
//         }
//         Err(_) => return failure_result,
//     }
// }


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core;
mod db;
use core::{services, types};
use tokio;

#[tokio::main]
async fn main() {
    db::init();
    let config = types::ServiceConfig {
        name: "My Service".to_string(),
        description: "A sample service".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        secure: Some(true),
        user: Some("admin".to_string()),
        pass: Some("".to_string()),
        interval: Some(1000),
        retry_interval: Some(5000),
        interval_timeout: Some(3000),
    };

    let res = db::insert_service(&config);

    // println!("Hello, world!");
    // let smtp_config = types::ServiceConfig {
    //     host: String::from("smtp.freesmtpservers.com"),
    //     port: 25,
    //     ..Default::default()
    // };
    // let result = services::verify_smtp(smtp_config);
    // match result.success {
    //     true => {
    //         println!("Success: {}", result.message)
    //     }
    //     false => {
    //         println!("Failed: {}", result.message)
    //     }
    // }

    // let website_config = types::ServiceConfig {
    //     host: String::from("https://www.army.mil.ng"),
    //     port: 80,
    //     ..Default::default()
    // };

    // let result = services::verify_website(website_config).await;
    // match result.success {
    //     true => {
    //         println!("Website > Success: {}", result.message)
    //     }
    //     false => {
    //         println!("Website > Failed: {}", result.message)
    //     }
    // }
}

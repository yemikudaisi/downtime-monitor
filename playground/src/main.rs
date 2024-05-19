// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core;
mod db;

use tokio;

#[tokio::main]
async fn main() {
    let _ = db::create_tables();
    // let config = types::ServiceConfig {
    //     id: None,
    //     name: "Service 2".to_string(),
    //     description: "My service 2".to_string(),
    //     host: "localhost".to_string(),
    //     port: 8080,
    //     secure: Some(true),
    //     user: Some("admin".to_string()),
    //     pass: Some("".to_string()),
    //     interval: Some(1000),
    //     retry_interval: Some(5000),
    //     interval_timeout: Some(3000),
    // };

    // let res = db::insert_service(&config);
    // println!("Last inserted ID {}", res);
    // println!("Terminated");
    let res = db::get_service_by_id(1).unwrap();
    // let deserialized = serde_json::to_string(res);
    println!("{:#?}", serde_json::to_string(&res).unwrap());
}

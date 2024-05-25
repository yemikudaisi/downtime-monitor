// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core;
mod db;

use core::{
    schedule_manager::JobSchedulerManager,
    types::{ServiceParameters, ServiceVerificationResult},
};
use tokio;

#[tokio::main]
async fn main() {
    let mut manager = JobSchedulerManager::new().await;
    let mut b_params = get_test_service();
    b_params.host = "https://army.mil.bd".to_string();
    b_params.interval = Some(15);
    manager
        .add_service(service_a, get_test_service())
        .await
        .expect("Failed to add service");

    manager
        .add_service(service_b, b_params)
        .await
        .expect("Failed to add service");

    let scheduler = manager.clone();
    tokio::spawn(async move {
        scheduler.start().await.expect("Failed to start scheduler");
    });

    manager.shutdown_on_ctrl_c().await;
}

fn get_test_service() -> ServiceParameters {
    ServiceParameters {
        name: String::from("Test Service 1"),
        host: String::from("https://www.google.com"),
        port: 80,
        interval: Some(10),
        ..Default::default()
    }
}

fn service_a(s: ServiceParameters) -> ServiceVerificationResult {
    println!("I run every 10 seconds.");
    ServiceVerificationResult {
        success: true,
        message: format!("false: {}", s.host).to_string(),
    }
}

fn service_b(s: ServiceParameters) -> ServiceVerificationResult {
    println!("I run every 15 seconds.");
    ServiceVerificationResult {
        success: true,
        message: format!("false: {}", s.host).to_string(),
    }
}

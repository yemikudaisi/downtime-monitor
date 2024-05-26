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
    let mut a_params = get_test_service();
    a_params.id = Some(1);
    let mut b_params = get_test_service();
    b_params.id = Some(2);
    b_params.host = "https://army.mil.bd".to_string();
    b_params.interval = Some(20);
    manager
        .add_service(service_a, a_params, notifier)
        .await
        .expect("Failed to add service");

    manager
        .add_service(service_b, b_params, notifier)
        .await
        .expect("Failed to add service");

    let scheduler = manager.clone();
    tokio::spawn(async move {
        scheduler.start().await.expect("Failed to start scheduler");
    });

    manager.shutdown_on_ctrl_c().await;
}

fn notifier(result: ServiceVerificationResult) {
    println!("Notificiation: {}", result);
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
    //print!("{:?}", s);
    println!("I run every 10 seconds.",);
    ServiceVerificationResult {
        service_id: s.id.unwrap(),
        success: true,
        message: format!("{}", s.host).to_string(),
    }
}

fn service_b(s: ServiceParameters) -> ServiceVerificationResult {
    println!("I run every 20 seconds.");
    ServiceVerificationResult {
        service_id: s.id.unwrap(),
        success: true,
        message: format!("{}", s.host).to_string(),
    }
}

#[tokio::test]
#[should_panic]
async fn test_duplicate_service_id_panics() {
    let manager = JobSchedulerManager::new().await;
    let mut a_params = get_test_service();
    a_params.id = Some(1);
    let mut b_params = get_test_service();
    b_params.id = Some(1);
    b_params.host = "https://army.mil.bd".to_string();
    b_params.interval = Some(20);
    manager
        .add_service(service_a, a_params, notifier)
        .await
        .expect("Failed to add service");

    manager
        .add_service(service_b, b_params, notifier)
        .await
        .expect("Failed to add service");
}

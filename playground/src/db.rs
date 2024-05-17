use dotenv::dotenv;
use rusqlite::{Connection, Error};
use std::env;

use crate::core::types::ServiceConfig;

pub fn establish_connection() -> Connection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::open(database_url).unwrap()
}

pub fn init() {
    let sql = "CREATE TABLE IF NOT EXISTS service_configs (
            name TEXT,
            description TEXT,
            host TEXT,
            port INTEGER,
            secure INTEGER,
            user TEXT,
            pass TEXT,
            interval INTEGER,
            retry_interval INTEGER,
            interval_timeout INTEGER
        )";
    let conn = establish_connection();
    let _ = conn.execute(sql, []);
    conn.close().unwrap();
}

pub fn insert_service(service: &ServiceConfig) -> rusqlite::Result<usize> {
    let conn = establish_connection();
    conn.execute(
        "INSERT INTO service_configs (name, description, host, port, secure, user, pass, interval, retry_interval, interval_timeout)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        &[
            &service.name, 
            &service.description, 
            &service.host, 
            &service.port.to_string(), 
            &service.secure.unwrap().to_string(), 
            &service.user.clone().unwrap().to_string(), 
            &service.pass.clone().unwrap().to_string(), 
            &service.interval.unwrap().to_string(), 
            &service.retry_interval.unwrap().to_string(), 
            &service.interval_timeout.unwrap().to_string()
            ],
    )
}

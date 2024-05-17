use dotenv::dotenv;
use rusqlite::{params, Connection, Error};
use std::env;

use crate::core::types::ServiceConfig;

pub fn get_connection() -> Connection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::open(database_url).expect("Unable to open a database connection")
}

pub fn close_connection(conn: Connection){
    conn.close().expect("Unable to closed databse connection.");
}

pub fn init() {
    let sql = "CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY,
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
    let conn = get_connection();
    let _ = conn.execute(sql, []).expect("Unable to create database");
    close_connection(conn);
}

/// .
/// Inserts service option into the database
/// # Panics
///
/// Panics if table doesn't exist.
///
/// # Errors
///
/// This function will return an error if .
pub fn insert_service(service: &ServiceConfig) -> i64 {
    let conn = get_connection();
    let _ = conn.execute(
        "INSERT INTO services (id, name, description, host, port, secure, user, pass, interval, retry_interval, interval_timeout)
         VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    );
    let res = conn.last_insert_rowid();
    close_connection(conn);
    res
}

pub fn get_service_config_id(id: i64) -> rusqlite::Result<ServiceConfig> {
    let conn = get_connection();
    let query = "SELECT id, name, description, host, port, secure, user, pass, interval, retry_interval, interval_timeout
                 FROM services
                 WHERE id = ?1";

    let row = conn.query_row(query, params![id], |row| {
        Ok(ServiceConfig {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            host: row.get(3)?,
            port: row.get(4)?,
            secure: Some(row.get::<_, String>(5)? == "true"), //.to_string() == "true" { Some(true) } else { Some(false) } ,
            user: row.get(6)?,
            pass: row.get(7)?,
            interval: row.get(8)?,
            retry_interval: row.get(9)?,
            interval_timeout: row.get(10)?,
        })
    });
    row
}

fn update_service_config( service: &ServiceConfig){
    let conn = get_connection();
    conn.execute(
        "UPDATE service_configs
         SET name = ?1, description = ?2, host = ?3, port = ?4, secure = ?5, user = ?6, pass = ?7, interval = ?8, retry_interval = ?9, interval_timeout = ?10
         WHERE id = ?11",
        params![
            service.name,
            service.description,
            service.host,
            service.port,
            service.secure,
            service.user,
            service.pass,
            service.interval,
            service.retry_interval,
            service.interval_timeout,
            service.id,
        ],
    ).expect("Unable to update value");
}


pub fn delete_service(id: &i64) {
    let conn = get_connection();
    conn.execute("DELETE FROM service_configs WHERE name = ?", &[id]).expect("Unable to delete service");
}

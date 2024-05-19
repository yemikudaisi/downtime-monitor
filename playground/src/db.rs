use chrono::Utc;
use dotenv::dotenv;
use rusqlite::{params, Connection, Error, Result};
use std::env;

use crate::core::types::ServiceConfig;

#[allow(unused)]
pub fn get_connection() -> Result<Connection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(Connection::open(database_url).expect("Unable to open a database connection"))
}

#[allow(unused)]
pub fn close_connection(conn: Connection){
    conn.close().expect("Unable to closed databse connection.");
}

#[allow(unused)]
pub fn create_tables() -> Result<usize, Error> {
    let query = "CREATE TABLE IF NOT EXISTS services (
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
            interval_timeout INTEGER,
            created_at TEXT,
            updated_at TEXT
        )";
    let conn = get_connection().unwrap();
    let result = conn.execute(query, []);
    close_connection(conn);
    result
}

#[allow(unused)]
pub fn delete_tables() -> Result<usize, Error> {
    let query = "DROP TABLE services";
    let conn = get_connection().unwrap();
    let result = conn.execute(query, []);
    close_connection(conn);
    result
}

/// .
/// Inserts service option into the database
/// # Panics
///
/// Panics if table doesn't exist.
///
/// # Errors
///
/// This function will return an error if
#[allow(unused)]
pub fn insert_service(service: &ServiceConfig) -> Result<i64> {
    let conn = get_connection().unwrap();
    let _ = conn.execute(
        "INSERT INTO services (id, name, description, host, port, secure, user, pass, interval, retry_interval, interval_timeout, created_at, updated_at)
         VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
            &service.interval_timeout.unwrap().to_string(),
            &Utc::now().to_rfc3339(),
            &Utc::now().to_rfc3339(),
            ],
    );
    let res = conn.last_insert_rowid();
    close_connection(conn);
    Ok(res)
}

#[allow(unused)]
pub fn get_service_by_id(id: i64) -> rusqlite::Result<ServiceConfig> {
    let conn = get_connection().unwrap();
    let query = "SELECT id, name, description, host, port, secure, user, pass, interval, retry_interval, interval_timeout, created_at, updated_at
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
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    });
    close_connection(conn);
    row
}

// fn main() -> Result<()> {
//     let conn = Connection::open("my_database.db")?;

//     let age: u8 = 25; // Replace with the desired age

//     let mut stmt = conn.prepare("SELECT id, username, email FROM users WHERE age = :age")?;
//     let user_iter = stmt.query_map(&[(":age", &age)], |row| {
//         Ok(User {
//             id: row.get(0)?,
//             username: row.get(1)?,
//             email: row.get(2)?,
//         })
//     })?;

//     for user in user_iter {
//         println!("{:?}", user);
//     }

//     Ok(())
// }
#[allow(unused)]
fn get_all_services() -> rusqlite::Result<Vec<ServiceConfig>> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM service_configs")?;
    let rows = stmt.query_map([], |row| {
        Ok(ServiceConfig {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            host: row.get(3)?,
            port: row.get(4)?,
            secure: Some(row.get::<_, String>(5)? == "true"),
            user: row.get(6)?,
            pass: row.get(7)?,
            interval: row.get(8)?,
            retry_interval: row.get(9)?,
            interval_timeout: row.get(10)?,            
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;
    rows.collect()
}

#[allow(unused)]
fn update_service( service: &ServiceConfig) -> rusqlite:: Result<usize>{
    let conn = get_connection().unwrap();
    let result = conn.execute(
        "UPDATE service_configs
         SET name = ?1, description = ?2, host = ?3, port = ?4, secure = ?5, user = ?6, pass = ?7, interval = ?8, retry_interval = ?9, interval_timeout = ?10, updated_at = ?11
         WHERE id = ?12",
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
            &Utc::now().to_rfc3339(),
            service.id,
        ],
    ).expect("Unable to update value");
    close_connection(conn);
    Ok(result)
}

#[allow(unused)]
pub fn delete_service(id: &i64) -> rusqlite::Result<usize> {
    let conn = get_connection().unwrap();
    let result = conn.execute("DELETE FROM service_configs WHERE name = ?", &[id]).expect("Unable to delete service");
    close_connection(conn);
    Ok(result)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_get_connection_is_ok(){
        let result = get_connection();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_database_tables_is_ok(){
        let result = create_tables();
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_database_tables_is_ok(){
        _ = create_tables();
        let result = delete_tables();
        assert!(result.is_ok());
        _ = create_tables();
    }

    #[test]
    fn test_insert_service_is_ok(){
        let _ = create_tables();
        let service_params = ServiceConfig {
            name: String::from("Test Service 1"),
            host: String::from("https://www.google.com"),
            port: 80,
            ..Default::default()
        };
        let result = insert_service(&service_params);
        match result {
            Ok(id) => {
                println!("The inserted ID is {}", id);
                assert!(true);
            },
            Err(e) => eprint!("{}", e)
        } 
    }
}
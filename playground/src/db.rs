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
    result
}

///
/// Gets the row count of a table
/// 
/// ## Panics
/// 
/// This funciton panics if the specified table is not in the database
/// 
#[allow(unused)]
fn get_table_count(table_name: &str) -> Result<i64> {
    let conn = get_connection().unwrap();
    let sql = format!("SELECT COUNT(*) FROM {}", table_name);
    let mut stmt = conn.prepare(&sql)?;
    stmt.clear_bindings();
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}


#[allow(unused)]
pub fn delete_tables() -> Result<usize, Error> {
    let query = "DROP TABLE services";
    let conn = get_connection().unwrap();
    let result = conn.execute(query, []);
    result
}


fn reset_tables(){
    _ = delete_tables();
    _ = create_tables();
}

/// .
/// Inserts service option into the database
/// 
/// ## Panics
///
/// Panics if table doesn't exist.
///
/// ## Errors
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
    let mut stmt = conn.prepare("SELECT * FROM services")?;
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
    
    Ok(result)
}

#[allow(unused)]
pub fn delete_service(id: &i64) -> rusqlite::Result<usize> {
    let conn = get_connection().unwrap();
    let result = conn.execute("DELETE FROM services WHERE id = ?", &[id]).expect("Unable to delete service");
    
    Ok(result)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn get_test_service() -> ServiceConfig{
        ServiceConfig {
            name: String::from("Test Service 1"),
            host: String::from("https://www.google.com"),
            port: 80,
            ..Default::default()
        }
    }

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
    fn test_table_table_count(){
        reset_tables();
        _ = insert_service(&get_test_service());
        let result = get_table_count("services");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }


    #[test]
    fn test_insert_service_is_ok(){
        reset_tables();
        let service_params = get_test_service();
        let result = insert_service(&service_params);
        assert!(result.is_ok());
        assert_eq!(get_table_count("services").unwrap(), 1);
    }

    #[test]
    fn test_get_service(){
        reset_tables();
        let _ = insert_service(&get_test_service());
        let result = get_service_by_id(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id.unwrap(), 1);
    }

    #[test]
    fn test_get_all_service(){
        reset_tables();
        let _ = insert_service(&get_test_service());
        let _ = insert_service(&get_test_service());
        let result = match get_all_services() {
            Ok(r) => r,
            Err(e) => { 
                eprint!("Error: {}", e);
                Vec::new()
            },
        };
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_delete_service(){
        reset_tables();
        let _ = insert_service(&get_test_service());
        let delete_result = delete_service(&1);
        assert!(delete_result.is_ok());
        let result = get_all_services();
        assert_eq!(result.unwrap().len(), 0);
    }
}
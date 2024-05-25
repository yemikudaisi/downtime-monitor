use chrono::Utc;
use dotenv::dotenv;
use rusqlite::{params, Connection, Result};
use std::env;

///
/// Creates a Rusqlite Connection
/// 
/// ## Panic
/// This function panics if and .env file does not exist
/// or DATABASE_URL is not specified in the .env file
#[allow(unused)]
pub fn get_connection() -> Result<Connection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(Connection::open(database_url).expect("Unable to open a database connection"))
}

#[allow(unused)]
pub fn create_tables() -> Result<()> {
    let service_query = "
        CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY,
            name TEXT,
            description TEXT,
            host TEXT,
            port INTEGER,
            secure TEXT,
            user TEXT,
            pass TEXT,
            interval INTEGER,
            retry_interval INTEGER,
            interval_timeout INTEGER,
            created_at TEXT,
            updated_at TEXT
        )";
    let heartbeat_query = "
        CREATE TABLE IF NOT EXISTS heartbeats (
            id INTEGER PRIMARY KEY,
            service_id INTEGER,
            status TEXT,
            time TEXT,
            msg TEXT,
            duration INTEGER,
            retries INTEGER
        )";
    let mut conn = get_connection().unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute(service_query, []).expect("Unable to create service table.");
    tx.execute(&heartbeat_query, []).expect("Unable to create service table.");
    tx.commit()
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
    let mut conn = get_connection().unwrap();
    let sql = format!("SELECT COUNT(*) FROM {}", table_name);
    let mut stmt = conn.prepare(&sql)?;
    stmt.clear_bindings();
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}

#[allow(unused)]
pub fn delete_tables() -> rusqlite::Result<()> {
    let mut conn = get_connection().unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute("DROP TABLE IF EXISTS heartbeats", []);
    tx.execute("DROP TABLE EXISTS services", []);
    tx.commit()
}

#[allow(unused)]
fn reset_tables(){
    _ = delete_tables();
    _ = create_tables();
}

pub mod heartbeat {
    use super::*;
    use crate::core::types::{Heartbeat, ServiceStatus};
    use std::str::FromStr;

    #[allow(unused)]
    pub fn insert(heartbeat: &Heartbeat) -> Result<()> {
        let conn = get_connection().unwrap();
        conn.execute(
            "INSERT INTO heartbeats (id, service_id, status, time, msg, duration, retries)
            VALUES (NULL, ?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                heartbeat.service_id,
                heartbeat.status.to_string(),
                heartbeat.time.to_string(),
                heartbeat.msg,
                heartbeat.duration,
                heartbeat.retries,
            ],
        )?;
        Ok(())
    }

    #[allow(unused)]
    pub fn get_by_id(id: i64) -> Result<Option<Heartbeat>> {
        let conn = get_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM heartbeats WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Heartbeat {
                id: row.get(0)?,
                service_id: row.get(1)?,
                status: ServiceStatus::from_str(&row.get::<_, String>(2)?).unwrap(),
                time:row.get(3)?,
                msg: row.get(4)?,
                duration: row.get(5)?,
                retries: row.get(6)?,
            }))
        } else {
            Ok(None)
        }
    }

    #[allow(unused)]
    pub fn get_all() -> Result<Vec<Heartbeat>> {
        let conn = get_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM heartbeats")?;
        let rows = stmt.query_map([], |row| {
            Ok(Heartbeat {
                id: row.get(0)?,
                service_id: row.get(1)?,
                status: ServiceStatus::from_str(&row.get::<_, String>(2)?).unwrap(),
                time: row.get(3)?,
                msg: row.get(4)?,
                duration: row.get(5)?,
                retries: row.get(6)?,
            })
        })?;

        rows.collect()
    }

    #[allow(unused)]
    pub fn update(heartbeat: &Heartbeat) -> Result<()> {
        let conn = get_connection().unwrap();
        conn.execute(
            "UPDATE heartbeats
            SET service_id = ?1, status = ?2, time = ?3, msg = ?4, duration = ?5, retries = ?6
            WHERE id = ?7",
            params![
                heartbeat.service_id,
                heartbeat.status.to_string(),
                heartbeat.time.to_string(),
                heartbeat.msg,
                heartbeat.duration,
                heartbeat.retries,
                heartbeat.id,
            ],
        )?;
        Ok(())
    }

    #[allow(unused)]
    pub fn delete(id: &i64) -> rusqlite::Result<usize> {
        let conn = get_connection().unwrap();
        let result = conn.execute("DELETE FROM heartbeats WHERE id = ?", &[id]).expect("Unable to delete heartbeat");
        
        Ok(result)
    }
}

pub mod service{
    use super::*;
    use crate::core::types::ServiceParameters;

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
    pub fn insert(service: &ServiceParameters) -> Result<i64> {
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
    pub fn get_by_id(id: i64) -> rusqlite::Result<ServiceParameters> {
        let conn = get_connection().unwrap();
        let query = "SELECT id, name, description, host, port, secure, user, pass, interval, retry_interval, interval_timeout, created_at, updated_at
                    FROM services
                    WHERE id = ?1";

        let row = conn.query_row(query, params![id], |row| {
            Ok(ServiceParameters {
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

    #[allow(unused)]
    pub fn get_all() -> rusqlite::Result<Vec<ServiceParameters>> {
        let conn = get_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM services")?;
        let rows = stmt.query_map([], |row| {
            Ok(ServiceParameters {
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
    pub fn update( service: &ServiceParameters) -> rusqlite:: Result<usize>{
        let conn = get_connection().unwrap();
        let result = conn.execute(
            "UPDATE services
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
    pub fn delete(id: &i64) -> rusqlite::Result<usize> {
        let conn = get_connection().unwrap();
        let result = conn.execute("DELETE FROM services WHERE id = ?", &[id]).expect("Unable to delete service");
        
        Ok(result)
    }

}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    #[allow(unused_imports)]
    use super::super::db::service::*;
    use crate::{core::types::{Heartbeat, ServiceParameters, ServiceStatus}, db::{create_tables, delete_tables, get_connection, get_table_count, reset_tables}};
    use crate::db::service;

    #[allow(unused)]
    fn get_test_service() -> ServiceParameters{
        ServiceParameters {
            name: String::from("Test Service 1"),
            host: String::from("https://www.google.com"),
            port: 80,
            ..Default::default()
        }
    }

    #[allow(unused)]
    fn get_test_heartbeat() -> Heartbeat{
        Heartbeat {
            id: None,
            service_id: 1,
            status: ServiceStatus::Up,
            time: Utc::now().to_rfc3339(),
            msg: "Pinged successful".to_string(),
            duration: 9,
            retries: 0,
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
        _ = crate::db::tests::insert(&get_test_service());
        let result = get_table_count("services");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
    
    #[cfg(test)]
    mod service_tests {
        #[allow(unused_imports)]
        use super::*;
        
        #[test]
        fn test_insert_service_is_ok(){
            reset_tables();
            let service_params = get_test_service();
            let result = service::insert(&service_params);
            assert!(result.is_ok());
            assert_eq!(get_table_count("services").unwrap(), 1);
        }
    
        #[test]
        fn test_get_service(){
            reset_tables();
            let _ = service::insert(&get_test_service());
            let result = service::get_by_id(1);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().id.unwrap(), 1);
        }
    
        #[test]
        fn test_get_all_service(){
            reset_tables();
            let _ = service::insert(&get_test_service());
            let _ = service::insert(&get_test_service());
            let result = match service::get_all() {
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
            let _ = service::insert(&get_test_service());
            let delete_result = service::delete(&1);
            assert!(delete_result.is_ok());
            let result = service::get_all();
            assert_eq!(result.unwrap().len(), 0);
        }
    
        #[test]
        fn test_update_service(){
            reset_tables();
            let mut service  = get_test_service();
            let insert_id = service::insert(&service).unwrap();
            service = service::get_by_id(insert_id).unwrap();
            let new_name = "New Service Name";
            service.name = new_name.to_string();
            let result  = service::update(&service);
            assert!(result.is_ok());
            service = service::get_by_id(insert_id).unwrap();
            assert_eq!(service.name, new_name.to_string());
        }
    }

     #[cfg(test)]
    mod heartbeat_tests {
        #[allow(unused_imports)]
        use crate::db::heartbeat;

        #[allow(unused_imports)]
        use super::*;
        
        #[test]
        fn insert_is_ok(){
            reset_tables();
            let heartbeat = get_test_heartbeat();
            let result = heartbeat::insert(&heartbeat);
            assert!(result.is_ok());
            assert_eq!(get_table_count("heartbeats").unwrap(), 1);
        }
    
        #[test]
        fn get_is_ok(){
            reset_tables();
            let _ = heartbeat::insert(&get_test_heartbeat());
            let result = heartbeat::get_by_id(1);
            assert!(result.is_ok());
            let id = result.unwrap().unwrap().id.unwrap();
            assert_eq!(id, 1);
        }
    
        #[test]
        fn get_all_is_ok(){
            reset_tables();
            let _ = heartbeat::insert(&get_test_heartbeat());
            let _ = heartbeat::insert(&get_test_heartbeat());
            let result = match heartbeat::get_all() {
                Ok(r) => r,
                Err(e) => { 
                    eprint!("Error: {}", e);
                    Vec::new()
                },
            };
            assert_eq!(result.len(), 2);
        }
    
        #[test]
        fn test_delete_is_ok(){
            reset_tables();
            let _ = heartbeat::insert(&get_test_heartbeat());
            let delete_result = heartbeat::delete(&1);
            assert!(delete_result.is_ok());
            let result = heartbeat::get_all();
            assert_eq!(result.unwrap().len(), 0);
        }
    }
}
use std::fs;
use std::path;

use rusqlite::config::DbConfig;
use rusqlite::{params, Connection, Error};
mod contact_type;
mod errors;
mod interview_type;
mod job_posting;
mod models;
mod utils;

fn enable_config_options(conn: &Connection) -> Result<(), Error> {
    let db_options = vec![
        DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY,
        DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER,
    ];

    for option in db_options {
        if !conn.db_config(option).unwrap() {
            let _ = conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
        }
    }
    Ok(())
}

fn get_path_to_sql_init_file() -> String {
    vec!["experimental", "testing.sql"].join(path::MAIN_SEPARATOR.to_string().as_str())
}

fn read_sql_file(path: &str) -> String {
    let sql_init_file_path = get_path_to_sql_init_file();

    // If path is an empty string, I want to use a default path
    let sql_file_path = if path.is_empty() {
        sql_init_file_path
    } else {
        let _path = if file_exist(path) {
            path.to_string()
        } else {
            sql_init_file_path
        };

        _path
    };

    let sql_file_string = fs::read_to_string(&sql_file_path).unwrap();
    sql_file_string
}

fn file_exist(path: &str) -> bool {
    let metadata = if let Ok(data) = fs::metadata(path) {
        Some(data)
    } else {
        None
    };

    let result = match metadata {
        Some(data) => {
            if data.is_file() {
                true
            } else {
                false
            }
        }
        None => false,
    };

    result
}

fn create_in_memory_db() -> Result<Connection, Error> {
    let conn = Connection::open_in_memory()?;
    let sql_stmts = read_sql_file("");
    let _ = conn.execute_batch(sql_stmts.as_str());

    let _ = enable_config_options(&conn)?;

    Ok(conn)
}

pub fn get_db_connection(path: &str) -> Result<Connection, Error> {
    let conn = if file_exist(path) {
        Connection::open(path)?
    } else {
        let _conn = Connection::open(path)?;

        // Get the sql needed to create the database
        let sql_stmts = read_sql_file("");
        let _ = _conn.execute_batch(sql_stmts.as_str());

        _conn
    };

    let _ = enable_config_options(&conn);

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_get_database_connection_with_existing_db() {
        let path_to_db =
            vec!["experimental", "test.db"].join(path::MAIN_SEPARATOR.to_string().as_str());

        let conn = get_db_connection(&path_to_db).unwrap();
        let mut stmt = conn
            .prepare("SELECT name FROM contacts where id=?;")
            .unwrap();

        let mut rows = stmt.query(params![1]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let name: String = row.get(0).unwrap();
            assert_eq!(name, "Marcus");
        }
    }

    #[test]
    fn test_enable_config_options() {
        let conn = get_db_connection("").unwrap();
        let _ = enable_config_options(&conn).unwrap();

        assert_eq!(
            conn.db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY)
                .unwrap(),
            true
        )
    }

    #[test]
    fn test_get_database_connection_with_new_db() {
        let file_name = "new_test_db.db";

        let conn = get_db_connection(file_name).unwrap();

        let mut stmt = conn.prepare("SELECT name FROM contacts;").unwrap();

        let _rows = stmt.query(params![]).unwrap();

        // Cleaning up the created file
        fs::remove_file(file_name).unwrap()
    }

    #[test]
    fn test_read_sql_file() {
        let real_sql_path = get_path_to_sql_init_file();

        let test_cases = vec!["", "src", real_sql_path.as_str()];

        for path in test_cases {
            let result = read_sql_file(path);
            assert_eq!(result.is_empty(), false);
        }
    }

    #[test]
    fn test_file_exist() {
        let test_cases = vec![("", false), ("src", false), ("Cargo.toml", true)];

        for (path, expected) in test_cases {
            let result = file_exist(path);

            assert_eq!(result, expected);
        }
    }
}

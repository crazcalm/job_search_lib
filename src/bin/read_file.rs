use std::fs::read_to_string;
use std::path;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let sql_file_path =
        vec!["experimental", "testing.sql"].join(path::MAIN_SEPARATOR.to_string().as_str());
    let sql_file_string = read_to_string(&sql_file_path)
        .unwrap_or_else(|_| panic!("Unable to find sql file at: {}", &sql_file_path));

    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(&sql_file_string).unwrap();

    Ok(())
}

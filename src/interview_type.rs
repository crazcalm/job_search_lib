use rusqlite::{params, Connection, Error};
use chrono::{DateTime, Local};
use chrono::prelude::*;

use crate::utils::{convert_option_string_to_option_date};
use crate::errors::JobSearchError;

#[derive(Debug)]
struct InterviewType {
    id: Option<i32>,
    name: String,
    last_updated: Option<DateTime<Local>>,
    hide: bool,
}

impl InterviewType {

    fn new(name: String) -> InterviewType{

        InterviewType{
            id: None,
            name,
            last_updated: None,
            hide: false
        }
    }

    fn new_from_db(id: i32, name: String, last_updated: Option<DateTime<Local>>, hide: i32) -> InterviewType {
        let hide = match hide {
            0 => false,
            _ => true,
        };

        InterviewType {
            id: Some(id),
            name,
            last_updated,
            hide,
        }
    }

    fn get_by_id(conn: &Connection, id: i32) -> Result<InterviewType, JobSearchError> {
        let interview_type = conn.query_row(
            "SELECT id, name, last_updated, hide FROM interview_types WHERE id = (?1)",
            params![id],
            |row| {
                let id: i32 = row.get(0)?;
                let name: String = row.get(1)?;
                let last_updated: Option<String> = match row.get(2)? {
                    Some(time) => Some(time),
                    None => None,
                };
                let last_updated = convert_option_string_to_option_date(last_updated);
                let hide: i32 = row.get(3)?;

                Ok(InterviewType::new_from_db(id, name, last_updated, hide))
            },
        )?;

        Ok(interview_type)
    }

    fn add_to_db(&mut self, conn: &Connection) -> Result<(), JobSearchError> {
        let _ = conn.execute(
            "INSERT INTO interview_types (name) VALUES (?1)",
            params![self.name],
        )?;

        // Update the id
        let id = conn.last_insert_rowid() as i32;

        let mut stmt =
            conn.prepare("SELECT name, last_updated, hide FROM interview_types WHERE id=(?1)")?;

        let row = stmt.query_row(params![id], |row| {
            let name: String = row.get(0)?;
            let last_updated: Option<String> = match row.get(1)? {
                Some(time) => Some(time),
                None => None,
            };
            let last_updated = convert_option_string_to_option_date(last_updated);
            let hide = match row.get(2)? {
                0 => false,
                _ => true,
            };

            Ok((name, last_updated, hide))
        })?;

        self.id = Some(id);
        self.name = row.0;
        self.last_updated = row.1;
        self.hide = row.2;

        Ok(())

    }

    fn update_db(){

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_in_memory_db;
    use chrono::prelude::*;

    #[test]
    fn test_new(){
        let name = "testing".to_string();

        let interview_type = InterviewType::new(name.clone());

        assert_eq!(interview_type.name, name);
    }

    #[test]
    fn test_get_by_id(){
        let conn = create_in_memory_db().unwrap();

        let name = "testing".to_string();
        let mut interview_type = InterviewType::new(name.clone());
        let _ = interview_type.add_to_db(&conn);

        let result = InterviewType::get_by_id(&conn, 1);
    }

    #[test]
    fn test_add_to_db(){
        let conn = create_in_memory_db().unwrap();

        let name = "testing".to_string();

        let mut interview_type = InterviewType::new(name.clone());
        interview_type.add_to_db(&conn).unwrap();

        assert_ne!(interview_type.id, None);
    }
}
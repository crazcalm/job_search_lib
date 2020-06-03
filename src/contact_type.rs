use chrono::prelude::*;
use chrono::{DateTime, Local};
use rusqlite::{params, Connection, Error};

use crate::errors::JobSearchError;
use crate::utils::convert_option_string_to_option_date;

#[derive(Debug)]
pub struct ContactType {
    id: Option<i32>,
    pub name: String,
    last_updated: Option<DateTime<Local>>,
    pub hide: bool,
}

impl ContactType {
    fn new(name: String) -> ContactType {
        ContactType {
            id: None,
            name,
            last_updated: None,
            hide: false,
        }
    }

    fn new_from_db(
        id: i32,
        name: String,
        last_updated: Option<DateTime<Local>>,
        hide: i32,
    ) -> ContactType {
        let hide = match hide {
            0 => false,
            _ => true,
        };

        ContactType {
            id: Some(id),
            name,
            last_updated,
            hide,
        }
    }

    fn get_by_id(conn: &Connection, id: i32) -> Result<ContactType, JobSearchError> {
        let contact_type = conn.query_row(
            "SELECT id, name, last_updated, hide FROM contact_types WHERE id = (?1)",
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

                Ok(ContactType::new_from_db(id, name, last_updated, hide))
            },
        );

        Ok(contact_type?)
    }

    fn add_to_db(&mut self, conn: &Connection) -> Result<(), JobSearchError> {
        let _ = conn.execute(
            "INSERT INTO contact_types (name) VALUES (?1)",
            params![self.name],
        )?;

        // Update the id
        let id = conn.last_insert_rowid() as i32;

        let mut stmt =
            conn.prepare("SELECT name, last_updated, hide FROM contact_types WHERE id=(?1)")?;

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

    fn update_db(&mut self, conn: &Connection) -> Result<(), JobSearchError> {
        let hide = match self.hide {
            true => 1,
            false => 0,
        };

        let _ = conn.execute(
            "UPDATE contact_types SET name = (?1), hide = (?2) WHERE id = (?3)",
            params![self.name, hide, self.id],
        )?;

        //need to update the last_updated field
        let last_updated = conn.query_row(
            "SELECT last_updated FROM contact_types WHERE id = (?1)",
            params![self.id],
            |row| {
                let last_updated: Option<String> = match row.get(0)? {
                    Some(time) => Some(time),
                    None => None,
                };
                let last_updated = convert_option_string_to_option_date(last_updated);

                Ok(last_updated)
            },
        )?;

        self.last_updated = last_updated;

        Ok(())
    }

    pub fn new_from_row(row: &rusqlite::Row) -> Result<ContactType, JobSearchError> {
        //add error handling

        let result = ContactType {
            id: row.get(0)?,
            name: row.get(1)?,
            last_updated: row.get(2)?,
            hide: match row.get(3)? {
                0 => false,
                _ => true,
            },
        };

        Ok(result)
    }

    fn get_all(conn: &Connection) -> Result<Vec<ContactType>, JobSearchError> {
        let mut stmt = conn.prepare("SELECT id, name, last_updated, hide FROM contact_types")?;

        let contact_types_iter =
            stmt.query_map(params![], |row| Ok(ContactType::new_from_row(row).unwrap()))?;

        let mut contact_types = Vec::new();

        for types in contact_types_iter {
            contact_types.push(types?);
        }

        Ok(contact_types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::create_in_memory_db;

    use chrono::prelude::*;

    #[test]
    fn test_new() {
        let name = "testing".to_string();

        let contact_type = ContactType::new(name.clone());

        assert_eq!(contact_type.id, None);
        assert_eq!(contact_type.name, name);
        assert_eq!(contact_type.last_updated, None);
        assert_eq!(contact_type.hide, false);
    }

    #[test]
    fn test_new_from_db() {
        let id: i32 = 5;
        let name = "testing".to_string();
        let last_updated = Local::now();
        let hide = 1; //everything other than 0 is true

        let contact_type =
            ContactType::new_from_db(id, name.clone(), Some(last_updated.clone()), hide);

        assert_eq!(contact_type.id, Some(id));
        assert_eq!(contact_type.name, name);
        assert_eq!(contact_type.last_updated, Some(last_updated));
        assert_eq!(contact_type.hide, true);
    }

    #[test]
    fn test_add_to_db() {
        let mut contact_type = ContactType::new("testing".to_string());

        let conn = create_in_memory_db().unwrap();

        let _ = contact_type.add_to_db(&conn).unwrap();

        assert_eq!(contact_type.id, Some(1));
    }

    #[test]
    fn test_get_by_id() {
        let mut contact_type = ContactType::new("testing".to_string());

        let conn = create_in_memory_db().unwrap();

        let _ = contact_type.add_to_db(&conn).unwrap();

        let contact_type_from_db = ContactType::get_by_id(&conn, contact_type.id.unwrap()).unwrap();

        assert_eq!(contact_type.id, contact_type_from_db.id)
    }

    #[test]
    fn test_update_db() {
        let mut contact_type = ContactType::new("testing".to_string());
        let conn = create_in_memory_db().unwrap();
        let _ = contact_type.add_to_db(&conn).unwrap();

        contact_type.hide = true;

        let _ = contact_type.update_db(&conn).unwrap();

        assert_ne!(contact_type.last_updated, None);
    }

    #[test]
    fn test_get_all_when_none() {
        let conn = create_in_memory_db().unwrap();

        let items = ContactType::get_all(&conn).unwrap();

        assert_eq!(items.is_empty(), true);
    }

    #[test]
    fn test_get_all() {
        let mut contact_type = ContactType::new("testing".to_string());

        let conn = create_in_memory_db().unwrap();

        let _ = contact_type.add_to_db(&conn).unwrap();

        let all_contact_types = ContactType::get_all(&conn).unwrap();

        assert_eq!(all_contact_types.is_empty(), false);
    }
}

use chrono::{DateTime, Local};
use rusqlite::{params, Connection, NO_PARAMS};

use crate::errors::JobSearchError;
use crate::utils::convert_option_string_to_option_date;

#[derive(Debug)]
struct Company {
    id: Option<i32>,
    name: String,
    address: Option<String>,
    website: Option<String>,
    phone: Option<String>,
    created_date: Option<DateTime<Local>>,
    last_updated: Option<DateTime<Local>>,
    hide: bool,
}

#[allow(dead_code)]
impl Company {
    fn new(
        name: String,
        address: Option<String>,
        website: Option<String>,
        phone: Option<String>,
    ) -> Company {
        Company {
            id: None,
            name,
            address,
            website,
            phone,
            created_date: None,
            last_updated: None,
            hide: false,
        }
    }

    fn new_from_db(
        id: Option<i32>,
        name: String,
        address: Option<String>,
        website: Option<String>,
        phone: Option<String>,
        created_date: Option<DateTime<Local>>,
        last_updated: Option<DateTime<Local>>,
        hide: bool,
    ) -> Company {
        Company {
            id,
            name,
            address,
            website,
            phone,
            created_date,
            last_updated,
            hide,
        }
    }

    fn update_db(&mut self, conn: &Connection) -> Result<(), JobSearchError> {
        let hide = match self.hide {
            true => 1,
            false => 0,
        };

        let _ = conn.execute(
            "UPDATE companies SET name=(?1), address=(?2), website=(?3), phone=(?4), hide=(?5) WHERE id = (?6)",
            params![self.name, self.address, self.website, self.phone, hide, self.id]
        )?;

        let row = conn.query_row(
            "SELECT last_updated FROM companies WHERE id=(?1)",
            params![self.id],
            |row| {
                let last_updated: Option<String> = row.get(0)?;
                let last_updated = convert_option_string_to_option_date(last_updated);

                Ok(last_updated)
            },
        )?;

        self.last_updated = row;

        Ok(())
    }

    fn add_to_db(&mut self, conn: &Connection) -> Result<(), JobSearchError> {
        let hide = match self.hide {
            true => 1,
            false => 0,
        };

        let _ = conn.execute(
            "INSERT INTO companies (name, address, website, phone, hide) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![self.name, self.address, self.website, self.phone, hide]
        )?;

        let id = conn.last_insert_rowid() as i32;

        let row = conn.query_row(
            "SELECT created_date, last_updated FROM companies WHERE id=(?1)",
            params![id],
            |row| {
                let created_date: Option<String> = row.get(0)?;
                let created_date = convert_option_string_to_option_date(created_date);

                let last_updated: Option<String> = row.get(1)?;
                let last_updated = convert_option_string_to_option_date(last_updated);

                Ok((created_date, last_updated))
            },
        )?;

        self.id = Some(id);
        self.created_date = row.0;
        self.last_updated = row.1;

        Ok(())
    }

    #[allow(dead_code)]
    fn get_all(conn: &Connection) -> Result<Vec<Company>, JobSearchError> {
        let mut stmt = conn.prepare(
            "SELECT id, name, address, website, phone, created_date, last_updated, hide FROM companies")?;

        let companies_iter = stmt.query_map(NO_PARAMS, |row| {
            let hide = match row.get(8)? {
                0 => true,
                _ => false,
            };

            Ok(Company::new_from_db(
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                hide,
            ))
        })?;

        let mut companies_list = Vec::new();
        for company in companies_iter {
            companies_list.push(company?);
        }

        Ok(companies_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_in_memory_db;

    #[test]
    fn test_update_db() {
        let conn = create_in_memory_db().unwrap();

        let name = "testing".to_string();

        let mut company = Company::new(name, None, None, None);
        company.add_to_db(&conn).unwrap();
        company.update_db(&conn).unwrap();

        assert_ne!(company.last_updated, None);
    }

    #[test]
    fn test_add_to_db() {
        let conn = create_in_memory_db().unwrap();

        let name = "name".to_string();
        let address = "address".to_string();
        let website = "website".to_string();
        let phone = "555-555-5555".to_string();

        let mut company = Company::new(
            name.clone(),
            Some(address.clone()),
            Some(website.clone()),
            Some(phone.clone()),
        );

        let _ = company.add_to_db(&conn).unwrap();

        assert_ne!(company.created_date, None);
        assert_eq!(company.last_updated, None);
    }
}

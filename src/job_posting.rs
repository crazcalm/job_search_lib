use chrono::prelude::*;
use chrono::{DateTime, Local};
use rusqlite::{params, Connection, Error};

use crate::errors::JobSearchError;
use crate::utils::convert_option_string_to_option_date;

#[derive(Debug)]
pub struct JobPosting {
    pub id: Option<i32>,
    pub link: String,
    pub created_date: Option<DateTime<Local>>,
    pub last_updated: Option<DateTime<Local>>,
    pub description: Option<String>,
    pub hide: bool,
}

impl JobPosting {
    pub fn new(link: String) -> JobPosting {
        JobPosting {
            id: None,
            link,
            created_date: None,
            last_updated: None,
            description: None,
            hide: false,
        }
    }

    pub fn new_from_row(row: &rusqlite::Row) -> Result<JobPosting, JobSearchError> {
        let hide = match row.get(5)? {
            0 => false,
            _ => true,
        };

        let job_posting = JobPosting {
            id: row.get(0)?,
            link: row.get(1)?,
            created_date: row.get(2)?,
            last_updated: row.get(3)?,
            description: row.get(4)?,
            hide,
        };

        Ok(job_posting)
    }

    pub fn get_all_job_postings(conn: &Connection) -> Result<Vec<JobPosting>, JobSearchError> {
        let mut stmt = conn
            .prepare(
                "SELECT id, link, created_date, last_updated, description, hide FROM job_postings;",
            )
            .unwrap();

        let job_postings_iter = stmt
            .query_map(params![], |row| Ok(JobPosting::new_from_row(row).unwrap()))
            .unwrap();

        let mut job_postings = Vec::new();

        for posting in job_postings_iter {
            job_postings.push(posting.unwrap());
        }

        Ok(job_postings)
    }

    pub fn add_to_db(&mut self, conn: &Connection) -> Result<(), JobSearchError> {
        //If it has an id, do not add it to the database
        // because it already exists
        let mut result = 0;
        match self.id {
            Some(_id) => {
                //Could add logging
                // Do nothing
            }
            None => {
                let mut stmt = conn.prepare(
                    "INSERT INTO job_postings\
                 (link, last_updated, description, hide) \
                 VALUES (?1, ?2, ?3, ?4);",
                )?;

                result = stmt.insert(params![
                    self.link,
                    self.last_updated,
                    self.description,
                    self.hide
                ])?;
            }
        }

        self.id = Some(result as i32);

        Ok(())
    }

    fn update_db(&mut self, conn: &Connection) -> Result<(), JobSearchError> {
        let hide = match self.hide {
            true => 1,
            false => 0,
        };

        let _ = conn.execute(
            "UPDATE job_postings SET link = (?1), description = (?2), hide = (?3) WHERE id = (?4)",
            params![self.link, self.description, hide, self.id],
        )?;

        //need to update the last_updated field
        let last_updated = conn.query_row(
            "SELECT last_updated FROM job_postings WHERE id = (?1)",
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::create_in_memory_db;
    use rusqlite::Connection;

    fn create_job_posting_test_data(conn: &Connection) {
        JobPosting::new(String::from("google"))
            .add_to_db(conn)
            .unwrap();
        JobPosting::new(String::from("amazon"))
            .add_to_db(conn)
            .unwrap();
        JobPosting::new(String::from("mozilla"))
            .add_to_db(conn)
            .unwrap();
    }

    #[test]
    fn test_get_all_job_postings() {
        let conn = create_in_memory_db().unwrap();

        create_job_posting_test_data(&conn);

        let job_postings = JobPosting::get_all_job_postings(&conn).unwrap();

        assert_eq!(job_postings.len(), 3);
    }

    #[test]
    fn test_update_db() {
        let conn = create_in_memory_db().unwrap();

        create_job_posting_test_data(&conn);

        let mut job_postings = JobPosting::get_all_job_postings(&conn).unwrap();

        let job_posting = job_postings.first_mut().unwrap();

        let description = "Testing this out".to_string();
        job_posting.description = Some(description);

        job_posting.update_db(&conn).unwrap();

        assert_ne!(job_posting.last_updated, None);
        assert_ne!(job_posting.description, None);
    }
}

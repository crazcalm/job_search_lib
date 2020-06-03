use rusqlite::{params, Connection, Error};

#[derive(Debug)]
struct AppliedTo {
    id: Option<i32>,
    date_applied: String,
    last_updated: Option<String>,
    company_id: Option<i32>,
    job_posting_id: Option<i32>,
    contact_id: Option<i32>,
    hide: i32,
}

#[derive(Debug)]
struct Company {
    id: Option<i32>,
    name: String,
    address: Option<String>,
    website: Option<String>,
    phone: Option<String>,
    created_date: String,
    last_updated: Option<String>,
    hide: i32,
}

#[derive(Debug)]
struct Contacts {
    id: Option<i32>,
    name: String,
    created_date: String,
    last_updated: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    description: Option<String>,
    type_id: i32,
    hide: i32,
}



#[derive(Debug)]
struct Interviews {
    id: Option<i32>,
    interview_type_id: i32,
    created_date: String,
    last_updated: Option<String>,
    date: Option<String>,
    company_id: i32,
    contact_id: Option<i32>,
    job_posting_id: Option<i32>,
    description: Option<String>,
    hide: i32,
}

#[derive(Debug)]
pub struct JobPosting {
    pub id: Option<i32>,
    pub link: String,
    pub created_date: Option<String>,
    pub last_updated: Option<String>,
    pub description: Option<String>,
    pub hide: i32,
}

impl JobPosting {
    pub fn new(link: String) -> JobPosting {
        JobPosting {
            id: None,
            link,
            created_date: None,
            last_updated: None,
            description: None,
            hide: 0,
        }
    }

    pub fn new_from_row(row: &rusqlite::Row) -> JobPosting {
        //add error handling

        JobPosting {
            id: row.get(0).unwrap(),
            link: row.get(1).unwrap(),
            created_date: row.get(2).unwrap(),
            last_updated: row.get(3).unwrap(),
            description: row.get(4).unwrap(),
            hide: row.get(5).unwrap(),
        }
    }

    pub fn get_all_job_postings(conn: &Connection) -> Result<Vec<JobPosting>, Error> {
        let mut stmt = conn
            .prepare(
                "SELECT id, link, created_date, last_updated, description, hide FROM job_postings;",
            )
            .unwrap();

        let job_postings_iter = stmt
            .query_map(params![], |row| Ok(JobPosting::new_from_row(row)))
            .unwrap();

        let mut job_postings = Vec::new();

        for posting in job_postings_iter {
            job_postings.push(posting.unwrap());
        }

        Ok(job_postings)
    }

    pub fn add_to_db(self, conn: &Connection) -> Result<i32, Error> {
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

        Ok(result as i32)
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

        for job_posting in job_postings {
            println!("{:?}", job_posting);
        }
    }
}

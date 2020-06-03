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

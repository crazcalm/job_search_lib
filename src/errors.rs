use std::fmt::Debug;

#[derive(Debug)]
pub enum JobSearchError {
    DBError(rusqlite::Error),
    SQLError(rusqlite::types::FromSqlError),
}

impl From<rusqlite::Error> for JobSearchError {
    fn from(error: rusqlite::Error) -> JobSearchError {
        JobSearchError::DBError(error)
    }
}

impl From<rusqlite::types::FromSqlError> for JobSearchError {
    fn from(error: rusqlite::types::FromSqlError) -> JobSearchError {
        JobSearchError::SQLError(error)
    }
}

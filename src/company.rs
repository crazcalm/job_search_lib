use chrono::prelude::*;
use chrono::{DateTime, Local};
use rusqlite::{params, Connection, Error};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::create_in_memory_db;
    use rusqlite::Connection;

    #[test]
    fn test_testing() {
        assert_eq!(1, 1);
    }
}

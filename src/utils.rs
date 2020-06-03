use chrono::prelude::*;
use chrono::{DateTime, Local};
use std::fs;

pub fn convert_option_string_to_option_date(
    string_time: Option<String>,
) -> Option<DateTime<Local>> {
    let date_option = match string_time {
        Some(time) => match parse_time(time.as_str()) {
            Ok(date) => Some(date),
            Err(_err) => {
                //log error
                None
            }
        },
        None => None,
    };
    date_option
}

pub fn parse_time(date: &str) -> Result<DateTime<Local>, std::num::ParseIntError> {
    //"2020-05-14 21:16:39"

    let date_time_parts: Vec<&str> = date.split(" ").collect();
    let date_parts: Vec<u32> = date_time_parts[0]
        .split("-")
        .map(|part| part.parse::<u32>().unwrap())
        .collect();

    let time_parts: Vec<u32> = date_time_parts[1]
        .split(":")
        .map(|part| part.parse::<u32>().unwrap())
        .collect();

    let result = Local
        .ymd(date_parts[0] as i32, date_parts[1], date_parts[2])
        .and_hms(time_parts[0], time_parts[1], time_parts[2]);

    println!("{}", result.format("%Y-%m-%d %H:%M:%S"));

    Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_exist() {
        let test_cases = vec![("", false), ("src", false), ("Cargo.toml", true)];

        for (path, expected) in test_cases {
            let result = file_exist(path);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_time() {
        let string_time = "2020-05-14 21:16:39";

        let result = parse_time(string_time).unwrap();

        assert_eq!(result.year(), 2020);
    }
}

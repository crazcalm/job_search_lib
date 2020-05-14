use std::fs;

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
}

use std::process;

fn main() {
    let mut exit_code;

    //Pre-commit hook to run my unit tests
    let tests = process::Command::new("cargo").arg("test").status();

    match tests {
        Ok(exit) => {
            exit_code = exit.code().unwrap();
        }
        Err(err) => {
            eprintln!("Running the test caused an error: {:?}", err);
            process::exit(1);
        }
    }

    println!("\n\nRunning linter for knowledge purposes:\n\n");
    let linter = process::Command::new("cargo").arg("clippy").status();

    match linter {
        Ok(exit) => {
            exit_code = if exit_code == 0 {
                exit.code().unwrap()
            } else {
                exit_code
            };
        }
        Err(err) => {
            eprintln!("Running the clippy caused an error: {:?}", err);
            process::exit(1);
        }
    }

    process::exit(exit_code);
}

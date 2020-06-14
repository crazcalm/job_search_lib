use std::process;

fn main() {
    //Pre-commit hook to run my unit tests
    let tests = process::Command::new("cargo").arg("test").status();

    match tests {
        Ok(exit) => {
            process::exit(exit.code().unwrap());
        }
        Err(err) => {
            eprintln!("Running the test caused an error: {:?}", err);
            process::exit(1);
        }
    }
}

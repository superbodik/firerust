mod browser;
mod scenes;

use std::process::ExitCode;

fn main() -> ExitCode {
    match browser::run_browser::run_browser() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

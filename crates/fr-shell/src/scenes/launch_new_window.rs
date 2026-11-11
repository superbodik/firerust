use std::io;
use std::process::Command;

pub fn launch_new_window() -> io::Result<()> {
    let program = std::env::current_exe()?;
    Command::new(program).spawn().map(|_| ())
}

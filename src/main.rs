use std::process::{self};

fn main() {
    if let Err(e) = student_management::run() {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}

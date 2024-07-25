use std::process::{self};

mod student_management;

fn main() {
    if let Err(e) = student_management::run() {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}

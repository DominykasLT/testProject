use simple_app::process_input;
use std::env;
use std::io::{self, Read};

fn main() {
    if env::args().any(|arg| arg == "--version") {
        println!("simple_app version 1.2");
        return;
    }

    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", err);
        return;
    }
    process_input(&input);
}

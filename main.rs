// src/main.rs

use simple_app::process_input;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", err);
        return;
    }
    process_input(&input);
}

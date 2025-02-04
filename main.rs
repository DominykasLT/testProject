use simple_app::{process_input, print_usage};
use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--version") {
        println!("simple_app version 1.2");
        println!("return");
        return;
    }

    if args.iter().any(|arg| arg == "--help") {
        print_usage();
        println!("return");
        return;
    }

    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", err);
        return;
    }

    process_input(&input);
}

fn print_usage() {
    println!("Usage: simple_app [OPTIONS]");
    println!("Options:");
    println!("  ");
    println!("  --version   Show application version");
    println!("  --help      Show this help message");
    println!("  ");
    println!("  (input)     Provide input for processing");
}

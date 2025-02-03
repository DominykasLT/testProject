pub fn process_input(input: &str) {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        println!("No input provided.");
    } else if trimmed.starts_with("panic") {
        let number_part = trimmed.trim_start_matches("panic").trim();
        let index: usize = number_part.parse().unwrap_or(0);
        let array = [100, 200, 300];
        println!("Accessing  array at index {}: {}", index, array[index]);
    } else {
        println!("Echo: {}", trimmed);
    }
}

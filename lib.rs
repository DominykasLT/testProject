pub const VERSION: &str = "1.2";

pub fn process_input(input: &str) {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        println!("No input provided.");
    } else if trimmed == "version" {
        println!("simple_app version: {}", VERSION);
    } else if trimmed.starts_with("panic") {
        let number_part = trimmed.trim_start_matches("panic").trim();
        let index: usize = match number_part.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid index provided after 'panic': '{}'", number_part);
                return;
            }
        };
        let array = [100, 200, 300];
        println!("Accessing array at index {}: {}", index, array[index]);
    } else {
        println!("Echo: {}", trimmed);
    }
}

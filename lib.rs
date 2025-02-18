pub const VERSION: &str = "1.2";

pub fn process_input(input: &str) {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        println!("Noooo input provided.");
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
        let array = [100, 200, 300, 400];
        println!("Accessing array at index {}: {}", index, array[index]);
    } else if trimmed.starts_with("double") {
        let number_part = trimmed.trim_start_matches("double").trim();
        match double_number(number_part) {
            Some(result) => println!("Doubled value: {}", result),
            None => eprintln!("Invalid number provided after 'double': '{}'", number_part),
        }
    } else {
        println!("Echo: {}", trimmed);
    }
}

fn double_number(input: &str) -> Option<i32> {
    println!("Test");
    input.parse::<i32>().ok().map(|num| num * 2)
}

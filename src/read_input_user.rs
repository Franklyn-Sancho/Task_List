use std::io::{self, Write};

pub fn read_user_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

use std::io::{self, Write};

pub fn password() -> String {
    io::stdout().flush().expect("Failed to flush stdout");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name.trim().to_string()
}
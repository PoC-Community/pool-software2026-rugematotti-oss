use std::io::{self, Write};

pub fn what_is_your_name() -> String {
    print!("What is your name?");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name.trim().to_string()
}

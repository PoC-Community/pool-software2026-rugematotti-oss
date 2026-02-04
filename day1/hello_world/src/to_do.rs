use std::io::{self, Write};

pub fn to_do() -> String {
    print!("Tasks: 1. Hit the gym,2. Go for movies, 3. Dinner with family  which activity would you like to choose(pick a number)");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut todo = String::new();
    io::stdin()
        .read_line(&mut todo)
        .expect("Failed to read line");
    todo.trim().to_string()
}

use crate::highlight::highlight_pattern;
use std::fs;

pub fn search_in_file(pattern: &str, file_path: &str, line_numbers: bool, case_insensitive: bool) {
    let content = fs::read_to_string(file_path)
        .expect("Failed to read file");

    for (index, line) in content.lines().enumerate() {
        let is_match = if case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };

        if is_match {
               let colored_line = highlight_pattern(line, pattern, case_insensitive);

    if line_numbers {
        println!("{}: {}", index + 1, colored_line);
    } else {
        println!("{}", colored_line);
    }
        }
    }
}

pub fn highlight_pattern(line: &str, pattern: &str, case_insensitive: bool) -> String {
    if pattern.is_empty() {
        return line.to_string(); // nothing to highlight
    }

    if case_insensitive {
        let mut result = String::new();
        let mut remaining = line;

        let pattern_lower = pattern.to_lowercase();

        while let Some(pos) = remaining.to_lowercase().find(&pattern_lower) {
            // split into before match, match, and after match
            let start = &remaining[0..pos];
            let matched = &remaining[pos..pos + pattern.len()];
            let end = &remaining[pos + pattern.len()..];

            // append before + highlighted match
            result.push_str(start);
            result.push_str(&format!("\x1b[1;31m{}\x1b[0m", matched));

            // continue with the rest of the line
            remaining = end;
        }

        // append whatever is left
        result.push_str(remaining);
        result
    } else {
        // case-sensitive, just replace all
        line.replace(pattern, &format!("\x1b[1;31m{}\x1b[0m", pattern))
    }
}

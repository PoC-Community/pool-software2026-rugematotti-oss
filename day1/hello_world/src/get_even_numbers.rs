pub fn get_even_numbers(numbers: &[i32]) -> String {
    let mut even_numbers: Vec<i32> = numbers.iter().filter(|n| *n % 2 == 0).cloned().collect();

    even_numbers.sort();

    even_numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" - ")
}

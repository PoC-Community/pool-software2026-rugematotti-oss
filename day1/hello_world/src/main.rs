mod get_even_numbers;
mod hello_world;
mod what_is_your_name;
mod to_do;
fn main() {
    hello_world::hello_world();

    router::router();

    let name = what_is_your_name::what_is_your_name();
    println!("Hello, {}!", name);


    let name = to_do::to_do();
    println!("Hello, {}!", to_do);

    let numbers = vec![1, 3, 4, 5, 6, 13, 18, 21, 23, 24, 30];
    let even_numbers = get_even_numbers::get_even_numbers(&numbers);
    println!("{}", even_numbers);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_even_numbers_positive() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(get_even_numbers::get_even_numbers(&numbers), "2 - 4 - 6");
    }
}

#[test]

fn test_get_even_numbers_negative() {
    let numbers = vec![-1, -2, -3, -4, -5, -6];
    assert_eq!(get_even_numbers::get_even_numbers(&numbers), "-6 - -4 - -2");
}

#[test]

fn test_get_even_numbers_mixed() {
    let numbers = vec![1, -2, 3, -4, 5, -6];
    assert_eq!(get_even_numbers::get_even_numbers(&numbers), "-6 - -4 - -2");
}

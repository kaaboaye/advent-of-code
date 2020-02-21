fn main() {
    let inputs = vec![""]
}

fn is_ok(code: String) -> bool {
    assert_eq!(code.len(), 6);

    let mut iter = code.chars()
        .map(|d| d.to_digit(10).unwrap() as i64);

    let mut was_duplicated = false;
    let mut previous_digit = iter.next().unwrap();

    for digit in iter {
        if digit < previous_digit {
            return false;
        }

        if digit == previous_digit {
            was_duplicated = true;
        }
    }

    was_duplicated
}

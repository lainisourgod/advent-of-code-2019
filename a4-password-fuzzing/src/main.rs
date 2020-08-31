use std::cmp::Ordering::*;

fn main() {
    let good_passwords_count = (172930..683082)
        .filter(|password| password_is_good(*password))
        .count();
    println!("Total of {} good passwords", good_passwords_count);
}

fn password_is_good(password: u32) -> bool {
    let mut has_two_adjacent_same_digits = false;
    let mut not_decreasing = true;

    let digits: Vec<char> = password.to_string().chars().collect();
    for (current, next) in digits.iter().zip(digits.iter().skip(1)) {
        not_decreasing = not_decreasing
            && match current.cmp(next) {
                Less => true,
                Equal => {
                    has_two_adjacent_same_digits = true;
                    true
                }
                Greater => false,
            };
    }

    // println!("{} {}", not_decreasing, has_two_adjacent_same_digits);

    not_decreasing && has_two_adjacent_same_digits
}

#[test]
fn test_password_is_good() {
    let cases = vec![
        (111111, true),
        (223457, true),
        (223450, false),
        (123789, false),
    ];

    for case in cases {
        assert_eq!(password_is_good(case.0), case.1, "case failed: {:?}", case);
    }
}

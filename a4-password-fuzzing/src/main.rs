use std::cmp::Ordering::*;

fn main() {
    let good_passwords_count = (172930..683082)
        .filter(|password| password_is_good(*password))
        .count();
    println!("Total of {} good passwords", good_passwords_count);
}

fn password_is_good(password: u32) -> bool {
    // Set initial values for our good password flags
    let mut not_decreasing = true;
    let mut group_sizes: Vec<u32> = Vec::new();
    let mut group_size = 1;

    let digits: Vec<char> = password.to_string().chars().collect();
    for (current, next) in digits.iter().zip(digits.iter().skip(1)) {
        // Do not use match as we have common logic for Greater and Less patterns.
        let ordering = current.cmp(next);
        if ordering == Equal {
            group_size += 1;
        } else {
            // Store size of previous group
            group_sizes.push(group_size);

            // Reset the group
            group_size = 1;

            // Check that digits do not decrease
            if ordering == Greater {
                not_decreasing = false;
            }
        };
    }

    // Push size of the last group
    group_sizes.push(group_size);

    let has_group_of_only_two_adjacent_same_digits = group_sizes.contains(&2);

    #[cfg(test)]
    println!(
        "{} {}",
        not_decreasing, has_group_of_only_two_adjacent_same_digits
    );

    not_decreasing && has_group_of_only_two_adjacent_same_digits
}

#[test]
fn test_password_is_good() {
    let cases = vec![
        (112233, true),
        (111122, true), // has a group of exactly two adjacent digits
        (223457, true),
        (223450, false), // decrease at the end
        (123789, false), // no same adjacent digits
        (121212, false), // same digits are not adjacent
        (111111, false), // all six digits are same
        (123444, false), // 3 adjacent -- too much
    ];

    for case in cases {
        assert_eq!(password_is_good(case.0), case.1, "case failed: {:?}", case);
    }
}

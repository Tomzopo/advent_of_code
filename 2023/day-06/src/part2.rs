pub fn part_two(input: &str) -> i64 {
    let (time, distance) = parse_input(input);

    (0..time)
        .filter(|hold| hold * (time - hold) > distance)
        .count() as i64
}

pub fn parse_input(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();

    let time = lines
        .first()
        .and_then(|s| s.strip_prefix("Time:"))
        .map(|s| s.replace(' ', "").parse::<i64>())
        .unwrap_or_else(|| panic!("Failed to parse"))
        .unwrap();

    let distance = lines
        .get(1)
        .and_then(|s| s.strip_prefix("Distance:"))
        .map(|s| s.replace(' ', "").parse::<i64>())
        .unwrap_or_else(|| panic!("Failed to parse"))
        .unwrap();

    (time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_test() {
        struct TestCase {
            input: &'static str,
            expected: i64,
        }

        let test_cases = vec![
            TestCase {
                input: r#"Time:      7  15   30
Distance:  9  40  200"#,
                expected: 71503,
            },
            TestCase {
                input: include_str!("../input/input2.txt"),
                expected: 24655068,
            },
        ];

        for test_case in test_cases {
            assert_eq!(part_two(test_case.input), test_case.expected);
        }
    }
}

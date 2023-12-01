pub fn part_two(input: &str) -> i64 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> i64 {
    let num_vec = Vec::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut res = (0..line.len()).filter_map(|index| {
        let line_parser = &line[index..];
        let val = if line_parser.chars().next().unwrap().is_ascii_digit() {
            line_parser.chars().next().unwrap()
        } else {
            let mut val = ' ';
            for (num, conv) in &num_vec {
                if line_parser.starts_with(num) {
                    val = *conv
                }
            }
            val
        };
        val.to_digit(10)
    });

    let first = res.next().unwrap();
    match res.last() {
        Some(last) => format!("{first}{last}"),
        None => format!("{first}{first}"),
    }
    .parse::<i64>()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(part_two(input), 281)
    }

    #[test]
    fn test_lines() {
        struct TestCase {
            input: &'static str,
            expected: i64,
        }
        let test_cases = vec![
            TestCase {
                input: r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#,
                expected: 281,
            },
            // This case confused the shit out of me
            TestCase {
                input: "fivezg8jmf6hrxnhgxxttwoneg",
                expected: 51,
            },
        ];

        for test_case in test_cases {
            assert_eq!(part_two(test_case.input), test_case.expected);
        }
    }
}

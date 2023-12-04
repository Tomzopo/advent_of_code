pub fn part_two(input: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_test() {
        struct TestCase {
            input: &'static str,
            expected: bool,
        }

        let test_cases = vec![
            TestCase {
                input: "test",
                expected: true,
            },
            // TestCase {
            //     input: "test",
            //     expected: true,
            // },
        ];

        for test_case in test_cases {
            assert_eq!(part_two(test_case.input), test_case.expected);
        }
    }
}

pub fn part_one(input: &str) -> i64 {
    let (times, distances) = parse_input(input);

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            (0..*time)
                .filter(|hold| hold * (time - hold) > *distance)
                .count() as i64
        })
        .product()
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let lines: Vec<&str> = input.lines().collect();

    let times: Vec<i64> = lines
        .first()
        .expect("Failed")
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<i64>().expect("Failed conversion"))
        .collect();

    let distances: Vec<i64> = lines
        .get(1)
        .expect("Failed")
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<i64>().expect("Failed conversion"))
        .collect();

    (times, distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        struct TestCase {
            input: &'static str,
            expected: i64,
        }

        let test_cases = vec![
            TestCase {
                input: r#"Time:      7  15   30
Distance:  9  40  200"#,
                expected: 288,
            },
            TestCase {
                input: include_str!("../input/input1.txt"),
                expected: 3317888,
            },
        ];

        for test_case in test_cases {
            assert_eq!(part_one(test_case.input), test_case.expected);
        }
    }
}

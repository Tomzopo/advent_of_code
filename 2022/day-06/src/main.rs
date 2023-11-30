use std::collections::HashSet;

fn main() {
    let file = include_str!("../input/input.txt");
    let part1 = detect_signal(file);
    println!("Part 1: {}", part1);
    let part2 = detect_message(file);
    println!("Part 2: {}", part2);
}

fn detect_signal(input: &str) -> usize {
    detect(input, 4)
}

fn detect_message(input: &str) -> usize {
    detect(input, 14)
}

fn detect(input: &str, win_len: usize) -> usize {
    let input_buf: Vec<char> = input.chars().collect();

    let (res, _) = input_buf
        .windows(win_len)
        .enumerate()
        .find(|(_, win)| HashSet::<char>::from_iter(win.iter().cloned()).len() == win_len)
        .unwrap();

    res + win_len
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: &'static str,
        expected: usize,
    }

    #[test]
    fn detect_signal_test() {
        let test_cases = vec![
            TestCase {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected: 5,
            },
            TestCase {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected: 6,
            },
            TestCase {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected: 10,
            },
            TestCase {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected: 11,
            },
        ];

        for test_case in test_cases {
            assert_eq!(detect_signal(test_case.input), test_case.expected);
        }
    }

    #[test]
    fn detect_message_test() {
        let test_cases = vec![
            TestCase {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                expected: 19,
            },
            TestCase {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected: 23,
            },
            TestCase {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected: 23,
            },
            TestCase {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected: 29,
            },
            TestCase {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected: 26,
            },
        ];

        for test_case in test_cases {
            assert_eq!(detect_message(test_case.input), test_case.expected);
        }
    }

    #[test]
    fn check_solutions() {
        let input = include_str!("../input/input.txt");
        assert_eq!(detect_signal(input), 1544);
        assert_eq!(detect_message(input), 2145);
    }
}

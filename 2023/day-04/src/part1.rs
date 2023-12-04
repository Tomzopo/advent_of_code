pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let matching_cards = get_matching_cards(line).len() as u32;

            if matching_cards == 0 {
                0
            } else {
                2_i32.pow(matching_cards - 1) as u32
            }
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .sum()
}

pub fn get_card_numbers(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}

pub fn get_matching_cards(input: &str) -> Vec<u32> {
    let (player_card, score_card) = input.split_once(": ").unwrap().1.split_once("| ").unwrap();

    let player_numbers = get_card_numbers(player_card);
    let score_numbers = get_card_numbers(score_card);

    player_numbers
        .iter()
        .filter(|num| score_numbers.contains(num))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        struct TestCase {
            input: &'static str,
            expected: u32,
        }

        let test_cases = vec![
            TestCase {
                input: r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
                expected: 13,
            },
            TestCase {
                input: include_str!("../input/input1.txt"),
                expected: 27059,
            },
        ];

        for test_case in test_cases {
            assert_eq!(part_one(test_case.input), test_case.expected);
        }
    }
}

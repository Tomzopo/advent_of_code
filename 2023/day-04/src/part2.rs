use std::collections::HashMap;

use crate::part1::get_matching_cards;

pub fn part_two(input: &str) -> u32 {
    let mut card_counter: HashMap<u32, u32> = HashMap::new();

    input
        .lines()
        .enumerate()
        .map(|(card_no, line)| ((card_no + 1) as u32, get_matching_cards(line)))
        .for_each(|(card_no, matching_cards)| {
            card_counter.entry(card_no).or_insert(1);
            let bonus_cards_count = matching_cards.len() as u32;
            for n in (card_no + 1)..(card_no + bonus_cards_count + 1) {
                card_counter.insert(
                    n,
                    card_counter.get(&n).unwrap_or(&1) + card_counter[&card_no],
                );
            }
        });

    card_counter.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_test() {
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
                expected: 30,
            },
            TestCase {
                input: include_str!("../input/input2.txt"),
                expected: 5744979,
            },
        ];

        for test_case in test_cases {
            assert_eq!(part_two(test_case.input), test_case.expected);
        }
    }
}

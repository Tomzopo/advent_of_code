use std::ops::Range;

use crate::part1::{parse_garden, AlmanacMap};

#[derive(Debug)]
struct SeedRange {
    start: i64,
    range: i64,
}

impl SeedRange {
    fn get_range(&self) -> Range<i64> {
        self.start..self.start + self.range
    }
}

pub fn part_two(input: &str) -> i64 {
    let mut inputs = input.split("\n\n");
    let seed_ranges = parse_seed_ranges(inputs.next());
    let garden = parse_garden(inputs);

    *seed_ranges
        .iter()
        .map(|seed_range| {
            seed_range
                .get_range()
                .map(|seed| garden.get_mapping(seed).expect("Broken"))
                .min()
                .unwrap()
        })
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
}

fn parse_seed_ranges(input: Option<&str>) -> Vec<SeedRange> {
    input
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<i64>().expect("Conversion Error"))
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|chunk| SeedRange {
            start: *chunk.first().unwrap(),
            range: *chunk.get(1).unwrap(),
        })
        .collect::<Vec<SeedRange>>()
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
                input: r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
                expected: 46,
            },
            TestCase {
                input: include_str!("../input/input2.txt"),
                expected: 26829166,
            },
        ];

        for test_case in test_cases {
            assert_eq!(part_two(test_case.input), test_case.expected);
        }
    }
}

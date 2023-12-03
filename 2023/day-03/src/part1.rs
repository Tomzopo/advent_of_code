use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
enum Entry {
    Digit(i64),
    Symbol(char),
    None,
}

pub fn part_one(input: &str) -> i64 {
    let input = parse_input(input);

    let max_x: i64 = input.keys().map(|key| key.0).max().unwrap() as i64;
    let max_y: i64 = input.keys().map(|key| key.1).max().unwrap() as i64;

    let num_coords: HashSet<(i64, i64)> = (0..=max_y)
        .flat_map(|y| (0..=max_x).map(move |x| (x, y)))
        .filter(|&(x, y)| matches!(input.get(&(x, y)), Some(Entry::Symbol(_))))
        .flat_map(|(x, y)| {
            [
                (y - 1, x - 1),
                (y, x - 1),
                (y + 1, x - 1),
                (y - 1, x),
                (y + 1, x),
                (y - 1, x + 1),
                (y, x + 1),
                (y + 1, x + 1),
            ]
            .iter()
            .cloned()
            .filter(|&(row, column)| {
                row >= 0
                    && row <= max_y
                    && column >= 0
                    && column <= max_x
                    && matches!(input.get(&(column, row)), Some(Entry::Digit(_)))
            })
            .map(|(row, mut column)| {
                while column > 0 && matches!(input.get(&(column - 1, row)), Some(Entry::Digit(_))) {
                    column -= 1;
                }
                (column, row)
            })
            .collect::<HashSet<(i64, i64)>>()
        })
        .collect();

    num_coords
        .iter()
        .filter_map(|&(x, y)| {
            let mut s = String::new();
            let mut x = x;
            while x <= max_x {
                if let Some(Entry::Digit(d)) = input.get(&(x, y)) {
                    s += &*d.to_string();
                    x += 1;
                } else {
                    break;
                }
            }
            s.parse::<i64>().ok()
        })
        .sum()
}

fn parse_input(input: &str) -> BTreeMap<(i64, i64), Entry> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let entry = if c == '.' {
                    Entry::None
                } else if c.is_ascii_digit() {
                    Entry::Digit(c.to_digit(10).unwrap() as i64)
                } else {
                    Entry::Symbol(c)
                };

                ((x as i64, y as i64), entry)
            })
        })
        .collect::<BTreeMap<(i64, i64), Entry>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        struct TestCase {
            input: &'static str,
            expected: i64,
        }

        let test_cases = vec![
            TestCase {
                input: r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
                expected: 4361,
            },
            TestCase {
                input: include_str!("../input/input1.txt"),
                expected: 520019,
            },
        ];

        for test in test_cases {
            assert_eq!(part_one(test.input), test.expected);
        }
    }
}

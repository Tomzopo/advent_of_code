use std::cmp;

pub fn part_two(input: &str) -> i64 {
    let input = crate::part1::parse_input(input);

    input
        .into_iter()
        .map(|(_, rounds)| {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            rounds.iter().for_each(|round| {
                let pulls = round.split(", ");
                pulls.for_each(|pull| {
                    let (count, colour) = pull.split_once(' ').unwrap();

                    // println!("{count} {colour}");
                    match colour {
                        "red" => red = cmp::max(red, count.parse::<i64>().unwrap()),
                        "green" => green = cmp::max(green, count.parse::<i64>().unwrap()),
                        "blue" => blue = cmp::max(blue, count.parse::<i64>().unwrap()),
                        _ => panic!("Uhmm"),
                    }
                });
            });

            println!("Round Maxes: {red} {green} {blue}");
            println!("Power: {}", red * green * blue);
            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_test() {
        struct TestCase {
            input: &'static str,
            expected: i64,
        }

        let test_cases = vec![
            TestCase {
                input: r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
                expected: 2286,
            },
            TestCase {
                input: include_str!("../input/input1.txt"),
                expected: 83707,
            },
        ];

        for test in test_cases {
            assert_eq!(part_two(test.input), test.expected)
        }
    }
}

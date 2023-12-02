pub fn part_one(input: &str) -> i64 {
    let input = parse_input(input);

    input
        .into_iter()
        .filter_map(|(game_id, rounds)| {
            rounds
                .iter()
                .all(|round| {
                    let (mut red, mut green, mut blue) = (0, 0, 0);
                    let pulls = round.split(", ");
                    pulls.for_each(|pull| {
                        let (count, colour) = pull.split_once(' ').unwrap();
                        //println!("{count} {colour}");

                        match colour {
                            "red" => red += count.parse::<i64>().unwrap(),
                            "green" => green += count.parse::<i64>().unwrap(),
                            "blue" => blue += count.parse::<i64>().unwrap(),
                            _ => panic!("Uhmm"),
                        }
                    });

                    println!("Game ID:{game_id}\tRed{red}\tGreen{green}\tBlue{blue}");

                    red <= 12 && green <= 13 && blue <= 14
                })
                .then_some(game_id)
        })
        .sum()
}

pub fn parse_input(input: &str) -> Vec<(i64, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let (game_id, rounds) = line.split_once(": ").expect("Ooof");
            let game_id = game_id
                .strip_prefix("Game ")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let rounds: Vec<String> = rounds.split("; ").map(String::from).collect();
            //println!("{game_id} {rounds:?}");
            (game_id, rounds)
        })
        .collect()
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
                expected: 8,
            },
            TestCase {
                input: include_str!("../input/input1.txt"),
                expected: 2685,
            },
        ];

        for test in test_cases {
            assert_eq!(part_one(test.input), test.expected)
        }
    }
}

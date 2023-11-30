fn main() {
    let file = include_str!("../input/games.txt");

    let score = rock_paper_scissors_part_1(file);
    println!("Final score Part 1: {}", score);

    let score = rock_paper_scissors_part_2(file);
    println!("Final score Part 2: {}", score);
}

fn rock_paper_scissors_part_1(input: &str) -> i64 {
    // Rock (A/X) = 1, Paper (B/Y) = 2, Scissors (C/Z) = 3
    // Loss = 0, Draw = 3, Win = 6
    input
        .lines()
        .map(|game| {
            let split = game
                .split_whitespace()
                .map(move_as_score)
                .collect::<Vec<i64>>();
            if split[0] == split[1] {
                split[1] + 3
            } else if split[0] == 1 && split[1] == 3 {
                split[1]
            } else if split[0] < split[1] || (split[0] == 3 && split[1] == 1) {
                split[1] + 6
            } else {
                split[1]
            }
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum()
}

fn rock_paper_scissors_part_2(input: &str) -> i64 {
    // Rock (A) = 1, Paper (B) = 2, Scissors (C) = 3
    // X = Lose, Y = Draw, Z = Win
    // Loss = 0, Draw = 3, Win = 6
    input
        .lines()
        .map(|game| {
            let split = game.split_whitespace().collect::<Vec<&str>>();
            match split[1] {
                "X" => {
                    // Loss
                    if split[0] == "A" {
                        3
                    } else {
                        move_as_score(split[0]) - 1
                    }
                }
                "Y" => {
                    // Draw
                    move_as_score(split[0]) + 3
                }
                "Z" => {
                    // Win
                    if split[0] != "C" {
                        move_as_score(split[0]) + 1 + 6
                    } else {
                        1 + 6
                    }
                }
                _ => panic!("Invalid game"),
            }
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum()
}

fn move_as_score(move_char: &str) -> i64 {
    match move_char {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("Invalid game"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors_part_1() {
        let input = concat!("A Y\n", "B X\n", "C Z\n",);

        assert_eq!(rock_paper_scissors_part_1(input), 15);
    }

    #[test]
    fn test_rock_paper_scissors_part_2() {
        let input = concat!("A Y\n", "B X\n", "C Z\n",);

        assert_eq!(rock_paper_scissors_part_2(input), 12);
    }

    #[test]
    fn solutions() {
        let input = include_str!("../input/games.txt");

        assert_eq!(rock_paper_scissors_part_1(input), 13005);
        assert_eq!(rock_paper_scissors_part_2(input), 11373);
    }
}
